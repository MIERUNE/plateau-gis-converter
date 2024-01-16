"""
cargo run --example render_schema > schema.json
python3 schema_to_dot.py > features.dot
dot features.dot -T svg -o hoge.svg
"""

import json

with open("./schema.json") as f:
    schema = json.load(f)

print(
    """digraph G {
      rankdir="LR"
"""
)

ignore = set()
for ty_from, v in schema["types"].items():
    if v["type"] == "Data":
        ignore.add(ty_from)
    elif v["type"] == "Property":
        ig = True
        for av in v["members"]:
            if isinstance(av["ref"], dict):
                ty_to = av["ref"]["Named"]
                if schema["types"][ty_to]["type"] == "Feature":
                    ig = False
        if ig:
            ignore.add(ty_from)

for ty_from, v in schema["types"].items():
    if v["type"] in ["Feature"]:
        for av in v["attributes"].values():
            if isinstance(av["ref"], dict):
                ty_to = av["ref"]["Named"]
                if ty_to not in ignore:
                    print(f'    "{ty_from}" -> "{ty_to}"')
    elif v["type"] == "Property":
        for av in v["members"]:
            if isinstance(av["ref"], dict):
                ty_to = av["ref"]["Named"]
                if ty_to not in ignore:
                    print(f'    "{ty_from}" -> "{ty_to}";')

print("""}""")
