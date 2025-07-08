# python-core/query/config.py
import os

# Get absolute path to rust-core/index.db regardless of where this is run
BASE_DIR = os.path.dirname(os.path.abspath(__file__))
SQLITE_DB_PATH = os.path.abspath(os.path.join(BASE_DIR, "../../rust-core/index.db"))

CHROMA_COLLECTION_NAME = "file_embeddings"
EMBED_MODEL_NAME = "all-MiniLM-L6-v2"
