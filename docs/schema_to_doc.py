"""Experimental: Schema to Markdown

Usage:

$ cd ./docs/
$ cargo run -p nusamai -- ~/Desktop/PLATEAU/13100_tokyo23-ku_2022_citygml_1_2_op/udx/bldg/53392547_bldg_6697_2_op.gml --schema schema.json --sink gpkg --output out.dummy
$ python3 schema_to_doc.py
"""

import json
from typing import TextIO


def format_referenced_type(ref) -> str:
    match ref:
        case {"Named": type_desc}:
            return type_desc
        case {"JsonString": orignal_attr}:
            type_desc = format_referenced_type(orignal_attr["ref"])
            if orignal_attr.get("max_occurs", 1) != 1:
                type_desc = type_desc + "[]"
            return f"JSON (`{type_desc}`)"
        case basic_type_name:
            return str(basic_type_name)


def print_type(ty, f: TextIO):
    f.write("| field | type |\n")
    f.write("|-------|------|\n")
    for attr_name, attr in ty["attributes"].items():
        ref_type = format_referenced_type(attr["ref"])
        f.write(f"| {attr_name} | { ref_type } |\n")

    f.write("\n")


def print_property(ty, f: TextIO):
    f.write("以下のいずれかの型をとります：\n\n")
    for member in ty["members"]:
        name = member["ref"]["Named"]
        f.write(f"- {name}\n")
    f.write("\n")


def generate_docs(schema, f: TextIO):
    for ty_name, ty in schema["types"].items():
        f.write(f"## {ty_name}\n\n")

        match ty["type"]:
            case "Feature" | "Data":
                print_type(ty, f)
            case "Property":
                print_property(ty, f)
            case _:
                raise ValueError(f"Unknown type: {ty['type']}")


def main():
    with open("schema.json", encoding="utf-8") as f:
        schema = json.load(f)

    with open("all_in_one.md", "w", encoding="utf-8") as f:
        generate_docs(schema, f)


if __name__ == "__main__":
    main()
