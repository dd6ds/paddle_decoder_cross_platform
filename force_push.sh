#!/bin/bash
# Force Push to GitHub - All Methods

echo "╔════════════════════════════════════════════════════════════╗"
echo "║  Push to GitHub: dd6ds/paddle_decoder_cross_platform      ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

cd /home/developer/rust/paddle_decoder_cross_platform

# Check if there are uncommitted files
if [ -n "$(git status --porcelain)" ]; then
    echo "⚠️  You have uncommitted changes!"
    echo ""
    git status --short
    echo ""
    read -p "Add and commit these files? (y/n): " commit_choice
    if [ "$commit_choice" = "y" ]; then
        git add .
        read -p "Commit message: " msg
        git commit -m "$msg"
        echo "✓ Committed"
    fi
    echo ""
fi

echo "Choose push method:"
echo ""
echo "1. Regular Push (try this first)"
echo "2. Force Push (⚠️  OVERWRITES remote)"
echo "3. Force Push with Lease (safer force push)"
echo "4. Set upstream and push"
echo "5. Push all branches and tags"
echo ""
read -p "Enter choice (1-5): " choice

case $choice in
    1)
        echo ""
        echo "═══════════════════════════════════════════════════════════"
        echo "  Regular Push"
        echo "═══════════════════════════════════════════════════════════"
        echo ""
        git push origin main
        ;;
    
    2)
        echo ""
        echo "═══════════════════════════════════════════════════════════"
        echo "  ⚠️  FORCE PUSH (Dangerous!)"
        echo "═══════════════════════════════════════════════════════════"
        echo ""
        echo "⚠️  WARNING: This will OVERWRITE the remote repository!"
        echo "⚠️  All remote changes will be LOST!"
        echo ""
        read -p "Are you SURE? Type 'yes' to continue: " confirm
        if [ "$confirm" = "yes" ]; then
            git push --force origin main
        else
            echo "Cancelled."
            exit 1
        fi
        ;;
    
    3)
        echo ""
        echo "═══════════════════════════════════════════════════════════"
        echo "  Force Push with Lease (Safer)"
        echo "═══════════════════════════════════════════════════════════"
        echo ""
        echo "This is safer than --force because it checks if remote"
        echo "has changes you don't have locally."
        echo ""
        git push --force-with-lease origin main
        ;;
    
    4)
        echo ""
        echo "═══════════════════════════════════════════════════════════"
        echo "  Set Upstream and Push"
        echo "═══════════════════════════════════════════════════════════"
        echo ""
        git push -u origin main
        ;;
    
    5)
        echo ""
        echo "═══════════════════════════════════════════════════════════"
        echo "  Push All Branches and Tags"
        echo "═══════════════════════════════════════════════════════════"
        echo ""
        git push --all origin
        git push --tags origin
        ;;
    
    *)
        echo "Invalid choice. Exiting."
        exit 1
        ;;
esac

if [ $? -eq 0 ]; then
    echo ""
    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║  ✅ SUCCESS! Code pushed to GitHub!                       ║"
    echo "╚════════════════════════════════════════════════════════════╝"
    echo ""
    echo "🎉 View at: https://github.com/dd6ds/paddle_decoder_cross_platform"
    echo ""
else
    echo ""
    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║  ❌ Push Failed                                            ║"
    echo "╚════════════════════════════════════════════════════════════╝"
    echo ""
    echo "Common issues:"
    echo "• Authentication: Run ./github_auth_setup.sh"
    echo "• Remote has changes: Pull first with 'git pull origin main'"
    echo "• Repository doesn't exist: Create it on GitHub first"
    echo ""
    echo "For authentication help, see: GITHUB_AUTH_FIX.md"
fi
