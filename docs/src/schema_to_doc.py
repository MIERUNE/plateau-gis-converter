"""Experimental: Schema to Markdown

Usage:

$ cd ./docs/
$ cargo run -p nusamai -- ~/Desktop/PLATEAU/13100_tokyo23-ku_2022_citygml_1_2_op/udx/bldg/53392547_bldg_6697_2_op.gml --schema schema.json --sink gpkg --output out.dummy
$ python3 schema_to_doc.py
"""

import json
from typing import Any, TextIO

FILE_FORMAT_LIST = ["gpkg", "shapefile", "geojson", "czml", "kml", "mvt", "3dtiles"]

ORDER_MAP = {
    "_": -2,
    "gml": -1,
    "core": 0,
    "bldg": 1,
    "tran": 2,
    "luse": 3,
    "brid": 4,
    "tun": 5,
    "cons": 6,
    "frn": 7,
    "veg": 8,
    "wtr": 9,
    "dem": 10,
    "grp": 11,
    "gen": 12,
    "uro": 13,
    "urf": 14,
}

with open("../../nusamai/data/plateau_spec.json", encoding="utf-8") as f:
    spec = json.load(f)


def _to_anchor_id(name: str) -> str:
    return name.lower().replace(":", "").replace(" ", "-").replace("/", "-")


def _to_anchor_id(name: str) -> str:
    return (
        name.lower()
        .replace(":", "")
        .replace(" ", "-")
        .replace("/", "-")
        .replace("_", "-")
    )


def format_referenced_type(ref) -> str:
    match ref:
        case {"Named": type_desc}:
            return type_desc
        case {"JsonString": original_attr}:
            type_desc = format_referenced_type(original_attr["ref"])
            anchor = _to_anchor_id(type_desc)
            type_desc = f'<a href="#{anchor}">{type_desc}</a>'
            if original_attr.get("max_occurs", 1) != 1:
                type_desc += "[]"
            return f"JSON (<code>{type_desc}</code>)"
        case basic_type_name:
            return str(basic_type_name)


def get_attr_ja(ty_name: str, attr_name: str) -> str:
    if attr_name == "gen:genericAttribute":
        return "汎用属性"
    if type_spec := spec.get(ty_name):
        attr = type_spec["attributes"].get(attr_name)
        if attr:
            return attr.get("name_ja", "")
    return ""


def print_type(ty_name, ty, f: TextIO):
    f.write("| フィールド名 | 型 | 日本語名 | CityGML 属性名 |\n")
    f.write("|-----------|----|--------|---------------|\n")

    for attr_name, attr in ty["attributes"].items():
        ref_type = format_referenced_type(attr["ref"])
        original_name = attr.get("original_name") or attr_name
        ja = get_attr_ja(ty_name, original_name)
        f.write(f"| {attr_name} | {ref_type} | {ja} | { original_name} |\n")

    f.write("\n")


def print_property(ty, f: TextIO):
    f.write("以下のいずれかの型の値をとる：\n\n")
    for member in ty["members"]:
        name = member["ref"]["Named"]
        link = f"<a href='#{_to_anchor_id(name)}'>{name}</a>"
        f.write(f"- {link}\n")
    f.write("\n")


def name_sort_key(item: tuple[str, Any]):
    (key, _) = item
    ns, name = key.split(":")
    return (ORDER_MAP[ns], name)


def write_header(ty_name: str, f: TextIO):
    f.write(f"### {ty_name}\n\n")
    if ty_name in spec:
        ja = spec[ty_name]["name_ja"]
        f.write(f"{ja}\n")
    f.write("\n")


def generate_docs(schema, f: TextIO):
    f.write("## 地物 (Feature stereotype)\n\n")

    for ty_name, ty in sorted(schema["types"].items(), key=name_sort_key):
        if ty["type"] == "Feature":
            write_header(ty_name, f)
            print_type(ty_name, ty, f)

    f.write("## プロパティ (Property stereotype)\n\n")

    for ty_name, ty in sorted(schema["types"].items(), key=name_sort_key):
        if ty_name.startswith("_:"):
            continue
        if ty["type"] == "Property":
            f.write(f"### {ty_name}\n\n")
            print_property(ty, f)

    f.write("## データ (Data stereotype)\n\n")

    for ty_name, ty in sorted(schema["types"].items(), key=name_sort_key):
        if ty["type"] in "Data":
            write_header(ty_name, f)
            print_type(ty_name, ty, f)


def main():
    for file_format in FILE_FORMAT_LIST:
        try:
            with open(f"schema_{file_format}.json", encoding="utf-8") as f:
                schema = json.load(f)

            with open(
                f"../manual/mapping_rules/{file_format}.md", "w", encoding="utf-8"
            ) as f:
                generate_docs(schema, f)
        except FileNotFoundError:
            print(f"schema_{file_format}.json not found")
            pass


if __name__ == "__main__":
    main()
