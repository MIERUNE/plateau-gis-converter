use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{self, BufReader, Read};

#[derive(Debug)]
pub struct PublicHeader {
    // LAS 1.2
    pub file_signature: [u8; 4],
    pub file_source_id: u16,
    pub global_encoding: u16,
    pub project_id_guid_data_1: u32,
    pub project_id_guid_data_2: u16,
    pub project_id_guid_data_3: u16,
    pub project_id_guid_data_4: [u8; 8],
    pub version_major: u8,
    pub version_minor: u8,
    pub system_identifier: [u8; 32],
    pub generating_software: [u8; 32],
    pub file_creation_day_of_year: u16,
    pub file_creation_year: u16,
    pub header_size: u16,
    pub offset_to_point_data: u32,
    pub number_of_variable_length_records: u32,
    pub point_data_record_format: u8,
    pub point_data_record_length: u16,
    pub legacy_number_of_point_records: u32,
    pub legacy_number_of_points_by_return: [u32; 5],
    pub x_scale_factor: f64,
    pub y_scale_factor: f64,
    pub z_scale_factor: f64,
    pub x_offset: f64,
    pub y_offset: f64,
    pub z_offset: f64,
    pub x_max: f64,
    pub x_min: f64,
    pub y_max: f64,
    pub y_min: f64,
    pub z_max: f64,
    pub z_min: f64,
}

impl PublicHeader {
    pub fn new() -> PublicHeader {
        PublicHeader {
            file_signature: [0; 4],
            file_source_id: 0,
            global_encoding: 0,
            project_id_guid_data_1: 0,
            project_id_guid_data_2: 0,
            project_id_guid_data_3: 0,
            project_id_guid_data_4: [0; 8],
            version_major: 0,
            version_minor: 0,
            system_identifier: [0; 32],
            generating_software: [0; 32],
            file_creation_day_of_year: 0,
            file_creation_year: 0,
            header_size: 0,
            offset_to_point_data: 0,
            number_of_variable_length_records: 0,
            point_data_record_format: 0,
            point_data_record_length: 0,
            legacy_number_of_point_records: 0,
            legacy_number_of_points_by_return: [0; 5],
            x_scale_factor: 0.0,
            y_scale_factor: 0.0,
            z_scale_factor: 0.0,
            x_offset: 0.0,
            y_offset: 0.0,
            z_offset: 0.0,
            x_max: 0.0,
            x_min: 0.0,
            y_max: 0.0,
            y_min: 0.0,
            z_max: 0.0,
            z_min: 0.0,
        }
    }

    pub fn read_public_header(&mut self, reader: &mut BufReader<File>) -> io::Result<()> {
        reader.read_exact(&mut self.file_signature)?;

        self.file_source_id = reader.read_u16::<LittleEndian>()?;

        self.global_encoding = reader.read_u16::<LittleEndian>()?;

        self.project_id_guid_data_1 = reader.read_u32::<LittleEndian>()?;
        self.project_id_guid_data_2 = reader.read_u16::<LittleEndian>()?;
        self.project_id_guid_data_3 = reader.read_u16::<LittleEndian>()?;
        reader.read_exact(&mut self.project_id_guid_data_4)?;

        self.version_major = reader.read_u8()?;
        self.version_minor = reader.read_u8()?;

        reader.read_exact(&mut self.system_identifier)?;
        reader.read_exact(&mut self.generating_software)?;
        self.file_creation_day_of_year = reader.read_u16::<LittleEndian>()?;
        self.file_creation_year = reader.read_u16::<LittleEndian>()?;

        self.header_size = reader.read_u16::<LittleEndian>()?;

        self.offset_to_point_data = reader.read_u32::<LittleEndian>()?;

        self.number_of_variable_length_records = reader.read_u32::<LittleEndian>()?;

        self.point_data_record_format = reader.read_u8()?;

        self.point_data_record_length = reader.read_u16::<LittleEndian>()?;

        self.legacy_number_of_point_records = reader.read_u32::<LittleEndian>()?;
        self.legacy_number_of_points_by_return = [
            reader.read_u32::<LittleEndian>()?,
            reader.read_u32::<LittleEndian>()?,
            reader.read_u32::<LittleEndian>()?,
            reader.read_u32::<LittleEndian>()?,
            reader.read_u32::<LittleEndian>()?,
        ];

        self.x_scale_factor = reader.read_f64::<LittleEndian>()?;
        self.y_scale_factor = reader.read_f64::<LittleEndian>()?;
        self.z_scale_factor = reader.read_f64::<LittleEndian>()?;

        self.x_offset = reader.read_f64::<LittleEndian>()?;
        self.y_offset = reader.read_f64::<LittleEndian>()?;
        self.z_offset = reader.read_f64::<LittleEndian>()?;

        self.x_max = reader.read_f64::<LittleEndian>()?;
        self.x_min = reader.read_f64::<LittleEndian>()?;
        self.y_max = reader.read_f64::<LittleEndian>()?;
        self.y_min = reader.read_f64::<LittleEndian>()?;
        self.z_max = reader.read_f64::<LittleEndian>()?;
        self.z_min = reader.read_f64::<LittleEndian>()?;

        Ok(())
    }
}
