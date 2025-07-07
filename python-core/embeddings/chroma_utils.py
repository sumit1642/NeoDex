# python-core/embeddings/chroma_utils.py
import chromadb
from config import CHROMA_COLLECTION_NAME
from embedder import embed_metadata


def store_in_chroma():
    client = chromadb.PersistentClient(path="chroma_db")  # 👈 this line changed
    collection = client.get_or_create_collection(name=CHROMA_COLLECTION_NAME)

    metadata, embeddings = embed_metadata()

    documents = [
        f"{item['filename']} - {item['path']} - {item['filetype']}" for item in metadata
    ]

    ids = [f"file_{i}" for i in range(len(metadata))]

    collection.add(
        ids=ids,
        documents=documents,
        embeddings=[emb.tolist() for emb in embeddings],
        metadatas=metadata,
    )

    print(f"✅ Stored {len(metadata)} files in ChromaDB.")
