#!/bin/bash
# Quick GitHub Authentication Setup

echo "╔═══════════════════════════════════════════════════════════╗"
echo "║  GitHub Authentication Required                           ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""
echo "GitHub no longer accepts passwords for git operations."
echo "Choose your authentication method:"
echo ""
echo "1. SSH Key (Recommended - one-time setup)"
echo "2. Personal Access Token (Quick - easier for first time)"
echo ""
read -p "Enter choice (1 or 2): " choice

if [ "$choice" = "1" ]; then
    echo ""
    echo "═══════════════════════════════════════════════════════════"
    echo "  SSH Key Setup"
    echo "═══════════════════════════════════════════════════════════"
    echo ""
    echo "Your SSH public key:"
    echo "-----------------------------------------------------------"
    cat ~/.ssh/id_ed25519.pub
    echo "-----------------------------------------------------------"
    echo ""
    echo "📋 COPY THE KEY ABOVE"
    echo ""
    echo "Then follow these steps:"
    echo ""
    echo "1. Open in browser: https://github.com/settings/keys"
    echo "2. Click 'New SSH key'"
    echo "3. Title: 'T490s - Paddle Decoder'"
    echo "4. Key type: 'Authentication Key'"
    echo "5. Paste your key in the 'Key' field"
    echo "6. Click 'Add SSH key'"
    echo ""
    read -p "Press ENTER after you've added the key to GitHub..."
    
    echo ""
    echo "Testing SSH connection..."
    if ssh -T git@github.com 2>&1 | grep -q "successfully authenticated"; then
        echo "✅ SSH working!"
    else
        echo "⚠️  SSH test result:"
        ssh -T git@github.com 2>&1
        echo ""
        echo "If you see 'successfully authenticated' above, you're good!"
    fi
    
    echo ""
    echo "Now pushing to GitHub..."
    cd /home/developer/rust/paddle_decoder_cross_platform
    git push -u origin main
    
elif [ "$choice" = "2" ]; then
    echo ""
    echo "═══════════════════════════════════════════════════════════"
    echo "  Personal Access Token Setup"
    echo "═══════════════════════════════════════════════════════════"
    echo ""
    echo "Step 1: Generate Token"
    echo "-----------------------------------------------------------"
    echo "1. Open: https://github.com/settings/tokens"
    echo "2. Click 'Generate new token (classic)'"
    echo "3. Note: 'Paddle Decoder Push'"
    echo "4. Expiration: Your choice (30 days, 90 days, etc.)"
    echo "5. Select scopes:"
    echo "   ☑ repo (check all sub-boxes)"
    echo "6. Click 'Generate token' at bottom"
    echo "7. COPY THE TOKEN (you'll only see it once!)"
    echo ""
    read -p "Press ENTER after you've copied your token..."
    
    echo ""
    echo "Step 2: Switch to HTTPS"
    echo "-----------------------------------------------------------"
    cd /home/developer/rust/paddle_decoder_cross_platform
    git remote set-url origin https://github.com/dd6ds/paddle_decoder_cross_platform.git
    echo "✓ Remote set to HTTPS"
    
    echo ""
    echo "Step 3: Push to GitHub"
    echo "-----------------------------------------------------------"
    echo "When prompted:"
    echo "  Username: dd6ds"
    echo "  Password: [paste your token]"
    echo ""
    read -p "Press ENTER to push..."
    
    git push -u origin main
    
else
    echo "Invalid choice. Exiting."
    exit 1
fi

echo ""
if [ $? -eq 0 ]; then
    echo ""
    echo "╔═══════════════════════════════════════════════════════════╗"
    echo "║  ✅ SUCCESS! Repository pushed to GitHub!                ║"
    echo "╚═══════════════════════════════════════════════════════════╝"
    echo ""
    echo "🎉 Your code is now at:"
    echo "   https://github.com/dd6ds/paddle_decoder_cross_platform"
    echo ""
    echo "Next steps:"
    echo "1. Visit your repository"
    echo "2. Add description: 'CW Morse code trainer with interactive modes'"
    echo "3. Add topics: morse-code, ham-radio, rust, training"
    echo "4. Create v1.0.0 release with compiled binaries"
    echo ""
else
    echo ""
    echo "╔═══════════════════════════════════════════════════════════╗"
    echo "║  ❌ Push Failed                                           ║"
    echo "╚═══════════════════════════════════════════════════════════╝"
    echo ""
    echo "Common issues:"
    echo "• SSH: Key not added to GitHub (check steps above)"
    echo "• Token: Expired or wrong scope (needs 'repo' scope)"
    echo "• Repository doesn't exist on GitHub yet"
    echo ""
    echo "Run this script again to try again!"
fi
