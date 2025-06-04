#!/usr/bin/env python3
import http.server
import socketserver
import mimetypes
import os

# Add WASM MIME type
mimetypes.add_type('application/wasm', '.wasm')

class CustomHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        # Fix asset paths - redirect /frontend/assets/ to /assets/
        if self.path.startswith('/frontend/assets/'):
            self.path = self.path.replace('/frontend/assets/', '/assets/')
        
        return super().do_GET()
    
    def end_headers(self):
        # Add CORS headers for development
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', '*')
        
        # Ensure WASM files have correct MIME type
        if self.path.endswith('.wasm'):
            self.send_header('Content-Type', 'application/wasm')
        
        super().end_headers()

if __name__ == "__main__":
    PORT = 8080
    
    # Change to the build directory
    os.chdir('/Users/hbl/Documents/loco-platform/target/dx/frontend/release/web/public')
    
    with socketserver.TCPServer(("", PORT), CustomHTTPRequestHandler) as httpd:
        print(f"🚀 Frontend server running at http://localhost:{PORT}")
        print("✅ WASM MIME types configured")
        print("✅ Asset path fixes enabled")
        httpd.serve_forever()