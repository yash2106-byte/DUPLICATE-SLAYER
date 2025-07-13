# DUPLICATE-SLAYER🗂️
A fast, efficient, and user-friendly command-line tool written in Rust for finding and removing duplicate files on your system. This tool uses SHA-256 hashing to identify true duplicates and provides a clean interface for managing them.


✨ Features
-Fast Parallel Processing: Utilizes Rayon for multi-threaded file processing
-SHA-256 Hashing: Reliable duplicate detection using cryptographic hashing
-Progress Tracking: Real-time progress bars with ETA and processing speed
-Interactive Deletion: Safe prompt-based deletion with user confirmation
-Detailed Statistics: Shows wasted space and duplicate counts
-Cross-Platform: Works on Windows, macOS, and Linux


🚀 Installation
Prerequisites
-Rust (1.70 or later)
-Cargo package manager

Building from Source
bash
# Clone the repository
git clone <your-repo-url>
cd file-deduplicator

# Build the project
cargo build --release

# Run the executable
./target/release/file-deduplicator


Dependencies
This project uses the following crates:
toml
[dependencies]
sha2 = "0.10"
walkdir = "2.3"
rayon = "1.7"
indicatif = "0.17"


📖 Usage
Basic Usage
Run the program and follow the interactive prompts:
bash
cargo run

Example Session
Welcome to File deduplicator
Path to scan: /home/user/Documents

Searching directory: /home/user/Documents
1247 files to process
⠁ [00:00:03] [████████████████████████████████████████] 1247/1247 (00:00:00)

searching done
searching duplicates
deduplicate detection done

output
==========
Duplicate Group (Hash: a1b2c3d4...)
   Size: 2048576 bytes
[ORIGINAL] /home/user/Documents/photo1.jpg
[DUPLICATE] /home/user/Documents/backup/photo1.jpg
[DUPLICATE] /home/user/Documents/temp/photo1_copy.jpg


Summary
==========
Total duplicate files: 15
Total wasted space: 45231616 bytes (43.12 MB)

Do you want to delete duplicate files? (y/N): y
Deleted duplicate files
 Deleted: /home/user/Documents/backup/photo1.jpg
 Deleted: /home/user/Documents/temp/photo1_copy.jpg

removing complete!
Files deleted: 15
Space freed: 45231616 bytes (43.12 MB)
 

🛠️ How It Works
1. Directory Scanning
The tool recursively scans the specified directory using walkdir:
rust
let entries: Vec<_> = WalkDir::new(dir_path)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.file_type().is_file())
    .collect();
   
2. Hash Calculation
Each file is hashed using SHA-256 for reliable duplicate detection:
rust
fn calculate_hash(&self, path: &Path) -> Option<String> {
    let contents = fs::read(path).ok()?;
    let mut hasher = Sha256::new();
    hasher.update(&contents);
    Some(format!("{:x}", hasher.finalize()))
}

3. Duplicate Detection
Files with identical hashes are grouped together:
rust
let mut hash_groups: HashMap<String, Vec<FileInfo>> = HashMap::new();
for file in &self.files {
    hash_groups
        .entry(file.hash.clone())
        .or_insert_with(Vec::new)
        .push(file.clone());
}

4. Safe Deletion
The first file in each group is preserved as the "original", while subsequent files are marked as duplicates and can be safely deleted.
🔧 Configuration
Customizing the Tool
You can modify the behavior by editing the source code:

Change hash algorithm: Replace SHA-256 with another algorithm from the sha2 crate
Adjust progress bar style: Modify the ProgressStyle configuration
Add file filters: Extend the scanning logic to include/exclude specific file types

Example: Adding File Type Filtering
rust
.filter(|e| {
    e.file_type().is_file() && 
    e.path().extension()
        .map_or(false, |ext| ext == "jpg" || ext == "png")
})


⚠️ Important Notes
Safety Considerations

-Backup First: Always backup important files before running the deduplicator
-Review Results: Carefully review the duplicate groups before confirming deletion
-Symlinks: The tool follows symlinks, which may lead to unexpected behavior
-Permissions: Ensure you have proper permissions to delete files in the target directory


Performance Tips
-SSD Storage: Running on SSD storage will significantly improve performance
-Memory Usage: Large directories may consume substantial memory for hash storage
-Concurrent Processing: The tool automatically uses all available CPU cores


🐛 Troubleshooting
Common Issues
"Path doesn't exist!" Error
bash
# Ensure the path is correct and accessible
ls -la /path/to/directory

Permission Denied Errors
bash
# Run with appropriate permissions
sudo ./target/release/file-deduplicator

Out of Memory Errors

-Process smaller directories in batches
-Consider increasing system swap space


📊 Technical Details
Data Structure
The tool uses a custom FileInfo struct to store file metadata:
rust
#[derive(Debug, Clone)]
struct FileInfo {
    path: String,
    size: u64,
    hash: String,
}

Performance Characteristics
-Time Complexity: O(n) where n is the number of files
-Space Complexity: O(n) for storing file information and hashes
-I/O Bound: Performance primarily limited by disk read speed


 Acknowledgments
-Rayon: For excellent parallel processing capabilities
-Indicatif: For beautiful progress bars
-Walkdir: For efficient directory traversal
-SHA2: For reliable cryptographic hashing


⚡ Happy deduplicating! Remember to always backup your important files before running any deletion operations.
