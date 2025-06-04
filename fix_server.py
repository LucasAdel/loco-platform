#!/usr/bin/env python3
import http.server
import socketserver
import mimetypes
import os
import sys

# Add WASM MIME type
mimetypes.add_type('application/wasm', '.wasm')

class FixedHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        # Add CORS headers for development
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', '*')
        
        # Ensure WASM files have correct MIME type
        if self.path.endswith('.wasm'):
            self.send_header('Content-Type', 'application/wasm')
        
        super().end_headers()
    
    def log_message(self, format, *args):
        # Log requests to see what's being requested
        print(f"REQUEST: {args[0]} - {args[1]} - {args[2]}")

if __name__ == "__main__":
    PORT = 8080
    
    # Change to the build directory
    build_dir = '/Users/hbl/Documents/loco-platform/target/dx/frontend/release/web/public'
    os.chdir(build_dir)
    
    print(f"📂 Serving from: {os.getcwd()}")
    print(f"📁 Files available:")
    for root, dirs, files in os.walk('.'):
        for file in files:
            filepath = os.path.join(root, file)
            print(f"  {filepath}")
    
    with socketserver.TCPServer(("127.0.0.1", PORT), FixedHTTPRequestHandler) as httpd:
        print(f"🚀 Frontend server running at http://localhost:{PORT}")
        print("✅ WASM MIME types configured")
        print("✅ CORS headers enabled")
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\n👋 Server stopped")
            sys.exit(0)