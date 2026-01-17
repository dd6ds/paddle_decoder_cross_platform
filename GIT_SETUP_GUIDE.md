# Git Setup Guide for Paddle Decoder

## 🚀 Quick Start

### Initialize Git Repository

```bash
cd /home/developer/rust/paddle_decoder_cross_platform
git init
git add .
git commit -m "Initial commit: CW paddle decoder with training modes"
```

## 📋 What Gets Committed

### ✅ Source Files (COMMITTED)
```
src/
├── main.rs
├── cw_academy_training.rs
└── morse_player.rs
```

### ✅ Configuration (COMMITTED)
```
Cargo.toml
.cargo/config.toml
.gitignore
```

### ✅ Documentation (COMMITTED)
```
README.md
QUICKSTART.md
BUILD_*.md
TRAINING_MODE_ADDED.md
LISTENING_MODE_GUIDE.md
RANDOM_BLOCKS_GUIDE.md
*.md (all documentation)
```

### ✅ Build Scripts (COMMITTED)
```
build_all_platforms.sh
build_linux.sh
build_macos.sh
build_windows.sh
*.sh (all scripts)
*.ps1 (PowerShell scripts)
```

### ✅ Arduino Files (COMMITTED)
```
*.ino
INO_README.md
```

## 🚫 What Gets Ignored

### ❌ Build Artifacts (IGNORED)
```
target/                  # All Rust compilation
Cargo.lock              # Lock file
paddle_decoder*         # Compiled binaries
*.exe                   # Windows executables
release/*.zip           # Release archives
```

### ❌ OS Files (IGNORED)
```
.DS_Store              # macOS
Thumbs.db              # Windows
*~                     # Linux backups
```

### ❌ IDE Files (IGNORED)
```
.vscode/               # VSCode
.idea/                 # IntelliJ
*.swp                  # Vim
```

## 🔍 Check Before Committing

### See what will be committed:
```bash
git status
```

### See ignored files:
```bash
git status --ignored
```

### Check specific file:
```bash
git check-ignore -v target/
# Should show: .gitignore:2:target/    target/
```

## 📝 Recommended First Commit

```bash
# Initialize repo
git init

# Add all source files
git add .

# Check what's staged
git status

# Make initial commit
git commit -m "Initial commit: Paddle Decoder v1.0

Features:
- CW Academy training mode (Sessions 1-10)
- Listening practice with feedback
- Random blocks training mode
- Cross-platform support (Linux, Windows, macOS)
- MIDI paddle input support
- Audio tone generation
- Morse code decoder

Includes:
- Complete source code
- Build scripts for all platforms
- Comprehensive documentation
- Training guides and examples"
```

## 🌐 Add Remote (Optional)

### GitHub:
```bash
git remote add origin https://github.com/yourusername/paddle-decoder.git
git branch -M main
git push -u origin main
```

### GitLab:
```bash
git remote add origin https://gitlab.com/yourusername/paddle-decoder.git
git branch -M main
git push -u origin main
```

## 📊 Repository Size Estimate

### With .gitignore (RECOMMENDED):
```
Source code:        ~50 KB
Documentation:      ~200 KB
Build scripts:      ~20 KB
Configuration:      ~5 KB
─────────────────────────
Total:             ~275 KB  ✅ Small and clean!
```

### Without .gitignore (NOT RECOMMENDED):
```
Everything above:   ~275 KB
target/ folder:     ~500 MB  ❌
Binaries:           ~25 MB   ❌
OS files:           ~1 MB    ❌
─────────────────────────
Total:             ~526 MB  ❌ Way too large!
```

## 🎯 Best Practices

### DO:
✅ Commit source code (`.rs` files)
✅ Commit documentation (`.md` files)
✅ Commit build scripts (`.sh`, `.ps1`)
✅ Commit configuration (`Cargo.toml`)
✅ Commit `.gitignore` file
✅ Write descriptive commit messages

### DON'T:
❌ Commit `target/` folder
❌ Commit compiled binaries
❌ Commit `Cargo.lock` (for libraries, keep for binaries)
❌ Commit IDE settings
❌ Commit OS-specific files
❌ Commit temporary files

## 🔧 Useful Git Commands

### Check status:
```bash
git status
git status --short
git status --ignored
```

### Add files:
```bash
git add .                    # Add all (respects .gitignore)
git add src/                 # Add specific folder
git add *.md                 # Add documentation
```

### Commit:
```bash
git commit -m "Your message"
git commit -am "Add and commit"
```

### View history:
```bash
git log
git log --oneline
git log --graph --oneline --all
```

### Undo changes:
```bash
git checkout -- file.txt     # Discard changes
git reset HEAD file.txt      # Unstage file
git reset --soft HEAD^       # Undo last commit
```

## 🏷️ Recommended Tags

After committing, create version tags:

```bash
git tag -a v1.0.0 -m "Version 1.0.0
- Initial release
- CW Academy training mode
- Listening practice
- Random blocks mode"

git tag -a v1.1.0 -m "Version 1.1.0
- Added feature X
- Fixed bug Y"

# Push tags
git push origin --tags
```

## 📦 Release Management

### Create release branch:
```bash
git checkout -b release/v1.0
./build_all_platforms.sh
# Upload binaries to GitHub Releases
```

### Use GitHub Releases:
1. Go to GitHub repository
2. Click "Releases" → "Create new release"
3. Tag version (e.g., v1.0.0)
4. Upload compiled binaries:
   - `paddle_decoder_linux_amd64`
   - `paddle_decoder_win64_gnu.exe`
   - `paddle_decoder_x86_64_macOS`
   - etc.
5. Write release notes
6. Publish

## 🐛 Troubleshooting

### Accidentally committed large files:
```bash
git rm --cached target/
git commit -m "Remove target folder"
```

### Clean untracked files:
```bash
git clean -n              # Dry run (see what would be deleted)
git clean -f              # Delete untracked files
git clean -fd             # Delete untracked files and directories
```

### See ignored files:
```bash
git status --ignored
git ls-files --ignored --exclude-standard
```

### Check if file is ignored:
```bash
git check-ignore -v paddle_decoder_linux_amd64
# Output: .gitignore:10:paddle_decoder_linux_amd64
```

## 📚 Additional Resources

- [Git Documentation](https://git-scm.com/doc)
- [GitHub Guides](https://guides.github.com/)
- [Atlassian Git Tutorials](https://www.atlassian.com/git/tutorials)

---

**Ready to start?** Just run:
```bash
cd /home/developer/rust/paddle_decoder_cross_platform
git init
git add .
git commit -m "Initial commit"
```

73! 📻
