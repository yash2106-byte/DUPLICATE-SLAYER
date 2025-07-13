use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::io::{self, Write};
use sha2::{Sha256, Digest};
use walkdir::WalkDir;
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Debug, Clone)]
struct FileInfo {
    path: String,
    size: u64,
    hash: String,
}

struct FileDuplicator {
    files: Vec<FileInfo>,
    duplicates: HashMap<String, Vec<FileInfo>>,
}

impl FileDuplicator {
    fn new() -> Self {
        FileDuplicator {
            files: Vec::new(),
            duplicates: HashMap::new(),
        }
    }
    
    fn scan_directory(&mut self, dir_path: &str) {
        println!("Serching directory: {}", dir_path);
        let entries: Vec<_> = WalkDir::new(dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .collect();
            
        println!("{} files to process", entries.len());
        
        let pb = ProgressBar::new(entries.len() as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"));
             
        let files: Vec<FileInfo> = entries
            .par_iter()
            .filter_map(|entry| {
                pb.inc(1);
                self.process_file(entry.path())
            })
            .collect();
        
        pb.finish_with_message("searching done");
        self.files = files;
    }
    
    fn process_file(&self, path: &Path) -> Option<FileInfo> {
        let metadata = fs::metadata(path).ok()?;
        let size = metadata.len();
        if size == 0 {
            return None;
        }
        let hash = self.calculate_hash(path)?;
        
        Some(FileInfo {
            path: path.to_string_lossy().to_string(),
            size,
            hash,
        })
    }
    
    fn calculate_hash(&self, path: &Path) -> Option<String> {
        let contents = fs::read(path).ok()?;
        let mut hasher = Sha256::new();
        hasher.update(&contents);
        Some(format!("{:x}", hasher.finalize()))
    }
    
    fn find_duplicates(&mut self) {
        println!("searching duplicates");
        let mut hash_groups: HashMap<String, Vec<FileInfo>> = HashMap::new();
        
        for file in &self.files {
            hash_groups
                .entry(file.hash.clone())
                .or_insert_with(Vec::new)
                .push(file.clone());
        }
        
        self.duplicates = hash_groups
            .into_iter()
            .filter(|(_, files)| files.len() > 1)
            .collect();
        
        println!(" deduplicate detection done");
    }
    
    fn display_results(&self) {
        println!("\noutput");
        println!("==========");
        
        if self.duplicates.is_empty() {
            println!("no same files found!");
            return;
        }
        
        let mut total_wasted_space = 0u64;
        let mut duplicate_count = 0;
        
        for (hash, files) in &self.duplicates {
            println!("\n Duplicate Group (Hash: {}...)", &hash[..8]);
            println!("   Size: {} bytes", files[0].size);
            
            for (i, file) in files.iter().enumerate() {
                if i == 0 {
                    println!("[ORIGINAL] {}", file.path);
                } else {
                    println!("[DUPLICATE] {}", file.path);
                    total_wasted_space += file.size;
                    duplicate_count += 1;
                }
            }
        }
        
        println!("\n Summary");
        println!("==========");
        println!("Total duplicate files: {}", duplicate_count);
        println!("Total wasted space: {} bytes ({:.2} MB)", 
                 total_wasted_space, 
                 total_wasted_space as f64 / 1_048_576.0);
        self.prompt_for_deletion();
    }
    
    fn prompt_for_deletion(&self) {
        print!("\n  Do you want to delete duplicate files? (y/N): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        if input.trim().to_lowercase() == "y" {
            self.delete_duplicates();
        } else {
            println!(" No folders were deleted.");
        }
    }
    
    fn delete_duplicates(&self) {
        println!("Deleted duplicate files");
        
        let mut deleted_count = 0;
        let mut freed_space = 0u64;
        
        for files in self.duplicates.values() {
            for file in files.iter().skip(1) {
                match fs::remove_file(&file.path) {
                    Ok(_) => {
                        println!(" Deleted: {}", file.path);
                        deleted_count += 1;
                        freed_space += file.size;
                    }
                    Err(e) => {
                        println!("unable to delete {}: {}", file.path, e);
                    }
                }
            }
        }
        
        println!("\nremoving complete!");
        println!("Files deleted: {}", deleted_count);
        println!("Space freed: {} bytes ({:.2} MB)", 
                 freed_space, 
                 freed_space as f64 / 1_048_576.0);
    }
}

fn main() {
    println!("Welcome to File deduplicator");   
    let mut deduplicator = FileDuplicator::new();
    print!("Path to scan: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let directory = input.trim();
    
    if !Path::new(directory).exists() {
        println!("path doesn't exist!");
        return;
    }
    
    deduplicator.scan_directory(directory);
    deduplicator.find_duplicates();
    deduplicator.display_results();
}