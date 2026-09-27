use super::*;

pub struct Chunker {
    pub chunk_size: usize,
    pub chunk_overlap: usize,
}

impl Chunker {
    pub fn new(chunk_size: usize, chunk_overlap: usize) -> Self {
        Self {
            chunk_size,
            chunk_overlap,
        }
    }
    
    pub fn chunk_text(&self, text: &str, url: &str) -> Vec<Chunk> {
        let mut chunks = Vec::new();
        let words: Vec<&str> = text.split_whitespace().collect();
        
        let mut start = 0;
        while start < words.len() {
            let end = (start + self.chunk_size).min(words.len());
            let chunk_text: String = words[start..end].join(" ");
            
            let metadata = ChunkMetadata {
                url: url.to_string(),
                type_: "page".to_string(),
            };
            
            chunks.push(Chunk {
                id: uuid::Uuid::new_v4().to_string(),
                text: chunk_text,
                metadata,
            });
            
            start = end.saturating_sub(self.chunk_overlap);
        }
        
        chunks
    }
}
