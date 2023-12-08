use super::models::*;
use byteorder::{LittleEndian, WriteBytesExt};
use citygml::*;
use earcut_rs::{utils_3d::project3d_to_2d, Earcut};
use indexmap::IndexSet;
use nusamai_geometry::*;
use std::{collections::HashMap, io::Write};

#[derive(Debug, Clone)]
struct TopLevelCityObject {
    cityobj: FeatureOrData<'static>,
    geometries: Geometries,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Default, Clone)]
pub struct Geometries {
    pub vertices: Vec<[f64; 3]>,
    pub multipolygon: MultiPolygon<'static, 1, u32>,
    pub multilinestring: MultiPolygon<'static, 1, u32>,
}

#[derive(Debug, Clone)]
pub struct FeatureOrData<'a> {
    pub typename: &'a str,
    pub id: Option<&'a str>,
    pub attributes: HashMap<String, ObjectValue<'a>>,
}

#[derive(Debug, Clone)]
pub enum ObjectValue<'a> {
    String(&'a str),
    Code(&'a Code),
    Integer(i64),
    Double(f64),
    Measure(f64),
    Boolean(bool),
    URI(&'a URI),
    Date(&'a NaiveDate),
    Point(&'a Point),
    Array(Vec<ObjectValue<'a>>),
    FeatureOrData(FeatureOrData<'a>),
}

// 暫定で構造体を定義
#[derive(Debug, Clone, Default)]
struct Triangles {
    pub indices: Vec<u32>,
    pub vertices: IndexSet<[u32; 3]>,
    pub face_normals: Option<Vec<f32>>,
    pub vertex_normals: Option<Vec<f32>>,
    pub vertex_colors: Option<Vec<f32>>,
    pub vertex_ids: Option<Vec<u64>>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl Triangles {
    pub fn new(indices: Vec<u32>, vertices: IndexSet<[u32; 3]>) -> Self {
        Self {
            indices,
            vertices,
            ..Default::default()
        }
    }
}

fn tessellation(
    mpolys: &[MultiPolygon3],
    mu_lng: f64,
    mu_lat: f64,
) -> Result<Triangles, Box<dyn std::error::Error>> {
    let mut earcutter = Earcut::new();
    let mut buf3d: Vec<f64> = Vec::new();
    let mut buf2d: Vec<f64> = Vec::new();
    let mut triangles_out: Vec<u32> = Vec::new();

    let mut indices: Vec<u32> = Vec::new();
    let mut vertices: IndexSet<[u32; 3]> = IndexSet::new();

    for mpoly in mpolys.iter() {
        for poly in mpoly {
            let num_outer = match poly.hole_indices().first() {
                Some(&v) => v as usize,
                None => poly.coords().len() / 3,
            };

            buf3d.clear();
            buf3d.extend(poly.coords().chunks_exact(3).flat_map(|v| {
                let (lat, lng) = (v[0], v[1]);
                [
                    (lng - mu_lng) * (10000000. * lat.to_radians().cos() / 90.),
                    (lat - mu_lat) * (10000000. / 90.),
                    v[2],
                ]
            }));

            if project3d_to_2d(&buf3d, num_outer, &mut buf2d) {
                // earcut
                earcutter.earcut(&buf2d, poly.hole_indices(), 2, &mut triangles_out);
                // indices and vertices
                indices.extend(triangles_out.iter().map(|idx| {
                    let vbits = [
                        (buf3d[*idx as usize * 3] as f32).to_bits(),
                        (buf3d[*idx as usize * 3 + 1] as f32).to_bits(),
                        (buf3d[*idx as usize * 3 + 2] as f32).to_bits(),
                    ];
                    let (index, _) = vertices.insert_full(vbits);
                    index as u32
                }));
            } else {
                println!("WARN: polygon does not have normal");
            }
        }
    }

    return Ok(Triangles::new(indices, vertices));
}

fn make_binary_buffer(triangles: &Triangles) -> Vec<u8> {
    let indices = &triangles.indices;
    let vertices = &triangles.vertices;

    let mut indices_buf = Vec::new();
    let mut vertices_buf = Vec::new();

    // glTFのバイナリはリトルエンディアン
    for index in indices {
        indices_buf.write_u32::<LittleEndian>(*index).unwrap();
    }

    for vertex in vertices {
        for v in vertex {
            vertices_buf
                .write_f32::<LittleEndian>(f32::from_bits(*v))
                .unwrap();
        }
    }

    [&indices_buf[..], &vertices_buf[..]].concat()
}

fn make_glb(gltf_string: String, binary_buffer: Vec<u8>) -> Vec<u8> {
    // JSONチャンクをバイナリに変換し、4の倍数に調整
    let json_chunk = gltf_string.as_bytes();
    let json_chunk_len = json_chunk.len();
    let json_chunk_padded = {
        let mut v = json_chunk.to_vec();
        while v.len() % 4 != 0 {
            v.push(0); // 4バイト境界に合わせるために0でパディング
        }
        v
    };

    // JSONチャンクヘッダー
    // この長さはパディングを含まない元のJSONデータの長さ
    let json_chunk_header = [
        json_chunk_len as u32, // パディングなしの長さ
        0x4E4F534A,            // JSON (リトルエンディアンで "JSON")
    ];

    let binary_len = binary_buffer.len();

    // バイナリチャンクヘッダー
    let bin_chunk_header = [
        binary_len as u32,
        0x004E4942, // BIN (リトルエンディアンで "BIN")
    ];

    // ファイル全体の長さ
    // この長さはパディングを含む
    let total_length = 12 + 8 + json_chunk_padded.len() + 8 + binary_len;

    // GLBヘッダー
    let glb_header = [
        0x46546C67, // glTF (リトルエンディアンで "glTF")
        2,
        total_length as u32, // ファイル全体の長さ
    ];

    // ファイル作成前にバイナリを作成
    let mut glb = Vec::new();

    // ヘッダーの書き込み
    let _ = glb.write_all(&glb_header[0].to_le_bytes());
    let _ = glb.write_all(&glb_header[1].to_le_bytes());
    let _ = glb.write_all(&glb_header[2].to_le_bytes());

    // JSONチャンクの書き込み
    let _ = glb.write_u32::<LittleEndian>(json_chunk_header[0]);
    let _ = glb.write_u32::<LittleEndian>(json_chunk_header[1]);
    let _ = glb.write_all(&json_chunk_padded);

    // バイナリチャンクの書き込み
    let _ = glb.write_u32::<LittleEndian>(bin_chunk_header[0]);
    let _ = glb.write_u32::<LittleEndian>(bin_chunk_header[1]);
    let _ = glb.write_all(&binary_buffer);

    glb
}

fn make_gltf_json(triangles: &Triangles) -> String {
    let indices = &triangles.indices;
    let vertices = &triangles.vertices;

    // glTF のモデルを作成
    let mut gltf = Gltf::new();

    // glTF のアセットを作成
    let mut asset = Asset::new();
    asset.version = "2.0".to_string();

    gltf.asset = asset;

    // glTF のバッファを作成
    let mut buffer = Buffer::new();
    // indicesはu32なので4バイト、verticesはf32x3なので12バイト
    let indices_byte_length = indices.len() as u32 * 4;
    let vertices_byte_length = vertices.len() as u32 * 12;
    buffer.byte_length = indices_byte_length + vertices_byte_length;
    buffer.uri = Some("data.bin".to_string());

    gltf.buffers = Some(vec![buffer]);

    // glTF のバッファビューを作成
    let mut buffer_view1 = BufferView::new();
    buffer_view1.buffer = 0;
    buffer_view1.byte_length = indices_byte_length;
    buffer_view1.byte_offset = 0;
    buffer_view1.target = Some(BufferViewTarget::ElementArrayBuffer);

    let mut buffer_view2 = BufferView::new();
    buffer_view2.buffer = 0;
    buffer_view2.byte_length = vertices_byte_length;
    buffer_view2.byte_offset = indices_byte_length;
    buffer_view2.target = Some(BufferViewTarget::ArrayBuffer);

    gltf.buffer_views = Some(vec![buffer_view1, buffer_view2]);

    // glTF のアクセサを作成
    let mut accessor1 = Accessor::new();
    accessor1.buffer_view = Some(0);
    accessor1.byte_offset = 0;
    accessor1.component_type = ComponentType::UnsignedInt;
    accessor1.count = indices.len() as u32;
    accessor1.type_ = AccessorType::Scalar;
    let max_indices = indices.iter().max().unwrap();
    accessor1.max = Some(vec![*max_indices as f32]);
    accessor1.min = Some(vec![0.0]);

    let mut accessor2 = Accessor::new();
    accessor2.buffer_view = Some(1);
    accessor2.byte_offset = 0;
    accessor2.component_type = ComponentType::Float;
    accessor2.count = vertices.len() as u32;
    accessor2.type_ = AccessorType::Vec3;
    let mut max_vertex: [f32; 3] = [f32::MIN; 3];
    let mut min_vertex: [f32; 3] = [f32::MAX; 3];
    for vertex in vertices {
        for (i, v) in vertex.iter().enumerate() {
            let v = f32::from_bits(*v);
            if v > max_vertex[i] {
                max_vertex[i] = v;
            } else if v < min_vertex[i] {
                min_vertex[i] = v;
            }
        }
    }
    accessor2.max = Some(max_vertex.to_vec());
    accessor2.min = Some(min_vertex.to_vec());

    gltf.accessors = Some(vec![accessor1, accessor2]);

    // glTF のメッシュを作成
    let mut mesh = Mesh::new();
    let mut primitive1 = MeshPrimitive::new();
    primitive1.indices = Some(0);
    primitive1.mode = PrimitiveMode::Triangles;
    primitive1.attributes = {
        let mut map = HashMap::new();
        map.insert("POSITION".to_string(), 1);
        map
    };

    mesh.primitives = vec![primitive1];

    gltf.meshes = Some(vec![mesh]);

    // glTF のシーンを作成
    let mut scene = Scene::new();
    scene.nodes = Some(vec![0]);

    gltf.scenes = Some(vec![scene]);

    // glTF のノードを作成
    let mut node = Node::new();
    node.mesh = Some(0);

    gltf.nodes = Some(vec![node]);

    // glTF のシーンを設定
    gltf.scene = Some(0);

    gltf.to_string().unwrap()
}

fn calc_center(all_mpolys: &Vec<nusamai_geometry::MultiPolygon<'_, 3>>) -> (f64, f64) {
    // 中心の経緯度を求める
    let (mu_lat, mu_lng) = {
        let (mut mu_lat, mut mu_lng) = (0.0, 0.0);
        let mut num_features = 0;
        for mpoly in all_mpolys {
            let (mut feat_mu_lng, mut feat_mu_lat) = (0.0, 0.0);
            let mut num_verts = 0;
            for poly in mpoly {
                for v in poly.coords().chunks_exact(3) {
                    num_verts += 1;
                    feat_mu_lng += v[0];
                    feat_mu_lat += v[1];
                }
            }
            if num_verts > 0 {
                num_features += 1;
                mu_lat += feat_mu_lng / num_verts as f64;
                mu_lng += feat_mu_lat / num_verts as f64;
            }
        }
        (mu_lat / num_features as f64, mu_lng / num_features as f64)
    };
    println!("{} {}", mu_lat, mu_lng);
    (mu_lat, mu_lng)
}

fn main() {
    // 中心の経緯度を求める
    let (mu_lat, mu_lng) = calc_center(&all_mpolys);

    // 三角分割
    // verticesは頂点の配列だが、u32のビットパターンで格納されている
    let triangles = tessellation(&all_mpolys, mu_lng, mu_lat).unwrap();

    // バイナリバッファを作成
    let binary_buffer = make_binary_buffer(&triangles);
    fs::write("./data/data.bin", &binary_buffer).unwrap();

    // glTFのJSON文字列を作成
    let gltf_string = make_gltf_json(&triangles);
    fs::write("./data/data.gltf", &gltf_string).unwrap();

    // glbを作成
    let glb = make_glb(gltf_string, binary_buffer);

    // ファイルを作成
    let mut file = BufWriter::new(fs::File::create("./data/data.glb").unwrap());

    // ファイルの書き込み
    let _ = file.write_all(glb.as_slice());
    let _ = file.flush();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn generate_city_objects() -> Vec<TopLevelCityObject> {
        let mut top_level_city_object1 = generate_top_level_city_object();
        top_level_city_object1.cityobj.id = Some("bldg_test1");

        let mut top_level_city_object2 = generate_top_level_city_object();
        top_level_city_object2.cityobj.id = Some("bldg_test2");

        let city_objects = vec![
            top_level_city_object1.clone(),
            top_level_city_object2.clone(),
        ];

        city_objects
    }

    fn generate_top_level_city_object() -> TopLevelCityObject {
        let type_ = "Building";
        let id_ = Some("bldg_test");

        let mut attributes: HashMap<String, super::ObjectValue> = HashMap::new();
        let attribute_name = "building_id_attribute";
        let attribute_value = super::ObjectValue::String("bldg_test");
        attributes.insert(attribute_name.to_string(), attribute_value);

        let mut geometries = super::Geometries {
            vertices: Vec::new(),
            multipolygon: MultiPolygon::new(),
            multilinestring: MultiPolygon::new(),
        };
        geometries.vertices.push([0.0, 0.0, 0.0]);
        geometries.vertices.push([1.0, 0.0, 0.0]);
        geometries.vertices.push([1.0, 1.0, 0.0]);
        geometries.multipolygon.add_exterior(vec![[0], [1], [2]]);

        let cityobj = super::FeatureOrData {
            typename: type_,
            id: id_,
            attributes,
        };

        super::TopLevelCityObject {
            cityobj,
            geometries,
        }
    }

    #[test]
    fn test_generate_city_objects() {
        let city_objects = generate_city_objects();
        let top_level_city_object1 = &city_objects[0];
        let top_level_city_object2 = &city_objects[1];

        assert_eq!(top_level_city_object1.cityobj.id, Some("bldg_test1"));
        assert_eq!(top_level_city_object2.cityobj.id, Some("bldg_test2"));
    }

    #[test]
    fn test_make_binary_buffer() {}

    #[test]
    fn test_make_gltf() {}

    #[test]
    fn test_make_glb() {}
}
