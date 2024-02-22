"""Experimental: Schema to Markdown

Usage:

$ cd ./docs/
$ cargo run -p nusamai -- ~/Desktop/PLATEAU/13100_tokyo23-ku_2022_citygml_1_2_op/udx/bldg/53392547_bldg_6697_2_op.gml --schema schema.json --sink gpkg --output out.dummy
$ python3 schema_to_doc.py
"""

import json
from typing import Any, TextIO

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


def _to_anchor_id(name: str) -> str:
    return name.lower().replace(":", "").replace(" ", "-").replace("/", "-")


def format_referenced_type(ref) -> str:
    match ref:
        case {"Named": type_desc}:
            return type_desc
        case {"JsonString": orignal_attr}:
            type_desc = format_referenced_type(orignal_attr["ref"])
            anchor = _to_anchor_id(type_desc)
            type_desc = f'<a href="#{anchor}">{type_desc}</a>'
            if orignal_attr.get("max_occurs", 1) != 1:
                type_desc += "[]"
            return f"JSON (<code>{type_desc}</code>)"
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


def generate_docs(schema, f: TextIO):
    f.write("## Features \n\n")

    for ty_name, ty in sorted(schema["types"].items(), key=name_sort_key):
        if ty["type"] == "Feature":
            f.write(f"### {ty_name}\n\n")
            print_type(ty, f)

    f.write("## Properties \n\n")

    for ty_name, ty in sorted(schema["types"].items(), key=name_sort_key):
        if ty_name.startswith("_:"):
            continue
        if ty["type"] == "Property":
            f.write(f"### {ty_name}\n\n")
            print_property(ty, f)

    f.write("## Data \n\n")

    for ty_name, ty in sorted(schema["types"].items(), key=name_sort_key):
        if ty["type"] in "Data":
            f.write(f"### {ty_name}\n\n")
            print_type(ty, f)


def main():
    with open("schema.json", encoding="utf-8") as f:
        schema = json.load(f)

    with open("all_in_one.md", "w", encoding="utf-8") as f:
        generate_docs(schema, f)


if __name__ == "__main__":
    main()
