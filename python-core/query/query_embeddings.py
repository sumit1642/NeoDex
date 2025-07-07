from sentence_transformers import SentenceTransformer
import chromadb
from config import EMBED_MODEL_NAME, CHROMA_COLLECTION_NAME
import sys


def search_files(query, top_k=5):
    model = SentenceTransformer(EMBED_MODEL_NAME)
    query_embedding = model.encode(query)

    client = chromadb.PersistentClient(path="../embeddings/chroma_db/")
    collection = client.get_or_create_collection(name=CHROMA_COLLECTION_NAME)

    results = collection.query(query_embeddings=[query_embedding], n_results=top_k)

    docs = results["documents"][0]
    metas = results["metadatas"][0]
    distances = results["distances"][0]

    if not docs:
        print("❌ No results found.")
        return

    print("🔍 Top matches:\n")
    for i in range(len(docs)):
        meta = metas[i]
        distance = distances[i]
        similarity = (1 - distance) * 100  # convert to %

        print(f"{i+1}. 📄 {meta['filename']} ({meta['filetype']})")
        print(f"   📁 {meta['path']}")
        print(f"   📏 Size: {meta['size']} bytes | 📅 Modified: {meta['modified']}")
        print(f"   🔗 Similarity Score: {similarity:.2f}%\n")


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("❌ Please provide a search query")
        print('💡 Usage: python3 query_embeddings.py "your query here"')
        sys.exit(1)

    query = sys.argv[1]
    search_files(query)
