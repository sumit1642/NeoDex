# python-core/embeddings/embedder.py
from sentence_transformers import SentenceTransformer
from config import EMBED_MODEL_NAME
from loader import load_file_metadata


def embed_metadata():
    model = SentenceTransformer(EMBED_MODEL_NAME)
    metadata = load_file_metadata()

    texts = [
        f"{item['filename']} - {item['path']} - {item['filetype']}\nContent:\n{item['content']}"
        for item in metadata
    ]
    embeddings = model.encode(texts, convert_to_tensor=True)
    return metadata, embeddings
