#!/bin/bash
# Push to GitHub - Quick Script

echo "🚀 Pushing to GitHub: dd6ds/paddle_decoder_cross_platform"
echo ""
echo "⚠️  You will need to authenticate!"
echo ""
echo "Choose authentication method:"
echo "1. Personal Access Token (recommended for first-time)"
echo "2. SSH Key"
echo ""
read -p "Enter choice (1 or 2): " choice

if [ "$choice" = "1" ]; then
    echo ""
    echo "📝 Using HTTPS with Personal Access Token"
    echo ""
    echo "Steps:"
    echo "1. Get token from: https://github.com/settings/tokens"
    echo "2. Generate new token (classic)"
    echo "3. Check 'repo' scope"
    echo "4. Copy the token"
    echo ""
    echo "When prompted:"
    echo "  Username: dd6ds"
    echo "  Password: [paste your token]"
    echo ""
    read -p "Press Enter when ready to push..."
    
    git push -u origin main
    
elif [ "$choice" = "2" ]; then
    echo ""
    echo "🔑 Using SSH"
    echo ""
    echo "First, let's set remote to SSH..."
    git remote set-url origin git@github.com:dd6ds/paddle_decoder_cross_platform.git
    echo "✓ Remote updated to SSH"
    echo ""
    echo "Make sure you have:"
    echo "1. Generated SSH key: ssh-keygen -t ed25519"
    echo "2. Added to GitHub: https://github.com/settings/keys"
    echo ""
    read -p "Press Enter when ready to push..."
    
    git push -u origin main
    
else
    echo "Invalid choice. Exiting."
    exit 1
fi

echo ""
if [ $? -eq 0 ]; then
    echo "✅ Successfully pushed to GitHub!"
    echo ""
    echo "🎉 Your repository is now live at:"
    echo "   https://github.com/dd6ds/paddle_decoder_cross_platform"
    echo ""
    echo "Next steps:"
    echo "1. Visit your repository on GitHub"
    echo "2. Add a description and topics"
    echo "3. Create a release with compiled binaries"
else
    echo "❌ Push failed!"
    echo ""
    echo "Common issues:"
    echo "- Wrong token/password"
    echo "- Repository doesn't exist on GitHub"
    echo "- SSH key not configured"
    echo ""
    echo "Check GIT_CONFIGURED.md for detailed troubleshooting"
fi
