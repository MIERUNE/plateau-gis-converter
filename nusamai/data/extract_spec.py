"""3D都市モデル標準製品仕様書 別表「応用スキーマとXMLSchemaとの多重度の対応」 のExcelファイルをJSONに変換する

https://www.mlit.go.jp/plateau/file/libraries/doc/specification_attachedTable1.xlsx

このファイルには、各XML要素について、PLATEAUでの運用上の多重度のほか、対応する日本語表記などが記載されている。
"""

import json
import re

import pandas as pd

xls = pd.ExcelFile("./specification_attachedTable1.xlsx")
sheets = pd.read_excel(xls, sheet_name=None)

type_tree = {}

for sheet_name, df in sheets.items():
    print(sheet_name)
    if sheet_name == "版":
        continue

    df.rename(
        columns={
            "地物型／データ型": "type",
            "属性／関連役割\n※括弧で囲まれたグレーハッチのセルは、標準製品仕様書では対象外": "attribute",
            "日本語表記": "name_ja",
            "XMLSchema上の多重度": "schema_multiplexity",
            "運用上の\n多重度": "plateau_multiplexity",
            "運用上の多重度": "plateau_multiplexity",
            "留意事項": "note",
        },
        inplace=True,
    )

    for ri, row in df.iterrows():
        attr = row["attribute"]
        if pd.isna(attr):
            if row["type"] not in type_tree:
                type_tree[row["type"]] = {
                    "type": row["type"],
                    "name_ja": row["name_ja"],
                    "note": "" if pd.isna(row["note"]) else row["note"],
                    "attributes": {},
                }
        else:
            not_used_in_standard = False
            if m := re.match(r"\((.+)\)", attr):
                # 名前がカッコで囲まれている属性は標準製品仕様書では対象外
                not_used_in_standard = True
                attr = m.group(1)
            type_tree[row["type"]]["attributes"][attr] = {
                "name": attr,
                "schema_multiplexity": row["schema_multiplexity"],
                "plateau_multiplexity": row["plateau_multiplexity"],
                "not_used_in_standard": not_used_in_standard,
                "name_ja": row["name_ja"],
                "note": "" if pd.isna(row["note"]) else row["note"],
            }

with open("plateau_spec.json", "w") as f:
    json.dump(type_tree, f, ensure_ascii=False, indent=2)
