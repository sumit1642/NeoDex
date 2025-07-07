# python-core/embeddings/loader.py
import sqlite3
import os
from config import SQLITE_DB_PATH

SUPPORTED_EXTS = {".txt", ".md", ".log", ".csv", ".json", ".py"}


def load_file_metadata():
    conn = sqlite3.connect(SQLITE_DB_PATH)
    cursor = conn.cursor()

    cursor.execute(
        "SELECT path, filename, filetype, size, permissions, created, modified FROM files"
    )
    rows = cursor.fetchall()
    conn.close()

    keys = [
        "path",
        "filename",
        "filetype",
        "size",
        "permissions",
        "created",
        "modified",
    ]
    metadata = [dict(zip(keys, row)) for row in rows]

    for item in metadata:
        ext = os.path.splitext(item["filename"])[1].lower()
        if ext in SUPPORTED_EXTS:
            try:
                with open(item["path"], "r", encoding="utf-8", errors="ignore") as f:
                    item["content"] = f.read()
            except Exception:
                item["content"] = ""
        else:
            item["content"] = ""

    return metadata
