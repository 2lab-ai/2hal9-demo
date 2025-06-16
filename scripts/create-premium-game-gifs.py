#!/usr/bin/env python3
"""
Create high-quality GIF demos of Mini Go and Mini Hold'em games
"""

import asyncio
import os
import sys
from pathlib import Path
from playwright.async_api import async_playwright
import imageio
import numpy as np
from PIL import Image
import time

SCRIPT_DIR = Path(__file__).parent
PROJECT_ROOT = SCRIPT_DIR.parent
DEMO_DIR = PROJECT_ROOT / "demo"

async def capture_go_game():
    """Capture Mini Go gameplay"""
    async with async_playwright() as p:
        browser = await p.chromium.launch()
        page = await browser.new_page(viewport={'width': 800, 'height': 900})
        
        # Load the game
        await page.goto(f'file://{DEMO_DIR}/mini_go_premium.html')
        await page.wait_for_timeout(1000)
        
        frames = []
        
        # Capture initial state
        frames.append(await page.screenshot())
        
        # Simulate some moves
        moves = [
            (3, 3), (5, 5), (3, 5), (5, 3), (4, 4), 
            (6, 6), (2, 2), (6, 2), (2, 6), (4, 2),
            (4, 6), (7, 4), (1, 4), (4, 7), (4, 1)
        ]
        
        for i, (row, col) in enumerate(moves[:10]):
            # Click intersection
            await page.click(f'.intersection[data-row="{row}"][data-col="{col}"]')
            await page.wait_for_timeout(1500)  # Wait for AI move
            
            # Capture frame
            frames.append(await page.screenshot())
            
            # Show territory occasionally
            if i == 4 or i == 9:
                await page.click('button:has-text("Show Territory")')
                await page.wait_for_timeout(500)
                frames.append(await page.screenshot())
                await page.wait_for_timeout(2000)
        
        await browser.close()
        
        # Convert screenshots to GIF
        images = []
        for frame in frames:
            img = Image.open(frame)
            # Resize for smaller file size
            img = img.resize((600, 675), Image.Resampling.LANCZOS)
            images.append(np.array(img))
        
        # Save GIF
        output_path = DEMO_DIR / "mini_go_premium_demo.gif"
        imageio.mimsave(output_path, images, duration=2, loop=0)
        print(f"Created: {output_path}")
        
        return output_path

async def capture_holdem_game():
    """Capture Mini Hold'em gameplay"""
    async with async_playwright() as p:
        browser = await p.chromium.launch()
        page = await browser.new_page(viewport={'width': 1100, 'height': 800})
        
        # Load the game
        await page.goto(f'file://{DEMO_DIR}/mini_holdem_premium.html')
        await page.wait_for_timeout(1000)
        
        frames = []
        
        # Start new game
        await page.click('button:has-text("New Game")')
        await page.wait_for_timeout(1000)
        
        # Capture initial state
        frames.append(await page.screenshot())
        
        # Play through a hand
        actions = ['call', 'check', 'raise', 'call', 'check']
        
        for action in actions:
            # Wait for action buttons to appear
            await page.wait_for_selector('#action-buttons.show', timeout=5000)
            
            # Find and click the appropriate button
            try:
                if action == 'call':
                    await page.click('button.call:visible')
                elif action == 'check':
                    await page.click('button.check:visible')
                elif action == 'raise':
                    await page.click('button.raise:visible')
                elif action == 'fold':
                    await page.click('button.fold:visible')
            except:
                # If specific action not available, make any valid move
                await page.click('#action-buttons.show button:visible:first')
            
            await page.wait_for_timeout(2000)
            frames.append(await page.screenshot())
            
            # Wait for AI moves
            await page.wait_for_timeout(4000)
            frames.append(await page.screenshot())
        
        # Capture final showdown
        await page.wait_for_timeout(3000)
        frames.append(await page.screenshot())
        
        await browser.close()
        
        # Convert screenshots to GIF
        images = []
        for frame in frames:
            img = Image.open(frame)
            # Resize for smaller file size
            img = img.resize((825, 600), Image.Resampling.LANCZOS)
            images.append(np.array(img))
        
        # Save GIF
        output_path = DEMO_DIR / "mini_holdem_premium_demo.gif"
        imageio.mimsave(output_path, images, duration=2.5, loop=0)
        print(f"Created: {output_path}")
        
        return output_path

async def capture_game_showcase():
    """Create a combined showcase of multiple games"""
    async with async_playwright() as p:
        browser = await p.chromium.launch()
        
        frames = []
        
        # Capture each game briefly
        games = [
            ('mini_go_premium.html', 800, 900, 3),
            ('mini_holdem_premium.html', 1100, 800, 3),
            ('collective_maze.html', 900, 800, 2),
            ('consciousness_poker.html', 1000, 700, 2),
            ('swarm_optimization.html', 900, 700, 2)
        ]
        
        for game_file, width, height, duration in games:
            if not (DEMO_DIR / game_file).exists():
                continue
                
            page = await browser.new_page(viewport={'width': width, 'height': height})
            await page.goto(f'file://{DEMO_DIR}/{game_file}')
            await page.wait_for_timeout(1000)
            
            # Capture a few frames
            for _ in range(duration):
                frames.append(await page.screenshot())
                await page.wait_for_timeout(1000)
            
            await page.close()
        
        await browser.close()
        
        if frames:
            # Convert to GIF
            images = []
            for frame in frames:
                img = Image.open(frame)
                # Resize to consistent size
                img = img.resize((800, 600), Image.Resampling.LANCZOS)
                images.append(np.array(img))
            
            # Save showcase GIF
            output_path = DEMO_DIR / "genius_games_showcase.gif"
            imageio.mimsave(output_path, images, duration=1.5, loop=0)
            print(f"Created: {output_path}")
            
            return output_path

async def main():
    """Create all game GIFs"""
    print("🎮 Creating premium game demos...")
    
    # Create individual game GIFs
    try:
        go_gif = await capture_go_game()
        print("✅ Mini Go demo created")
    except Exception as e:
        print(f"❌ Failed to create Mini Go demo: {e}")
    
    try:
        holdem_gif = await capture_holdem_game()
        print("✅ Mini Hold'em demo created")
    except Exception as e:
        print(f"❌ Failed to create Mini Hold'em demo: {e}")
    
    try:
        showcase_gif = await capture_game_showcase()
        print("✅ Game showcase created")
    except Exception as e:
        print(f"❌ Failed to create showcase: {e}")
    
    print("\n🎉 All demos created successfully!")
    print("\nTo use in README:")
    print("![Mini Go Demo](demo/mini_go_premium_demo.gif)")
    print("![Mini Hold'em Demo](demo/mini_holdem_premium_demo.gif)")
    print("![Games Showcase](demo/genius_games_showcase.gif)")

if __name__ == "__main__":
    # Install required packages if needed
    try:
        import playwright
        import imageio
        import PIL
    except ImportError:
        print("Installing required packages...")
        os.system("pip install playwright imageio Pillow")
        os.system("playwright install chromium")
    
    asyncio.run(main())