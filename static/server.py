#!/usr/bin/env python3
import http.server
import socketserver
import mimetypes
import os
import json
from urllib.parse import urlparse, parse_qs

# Add WASM MIME type
mimetypes.add_type('application/wasm', '.wasm')
mimetypes.add_type('application/json', '.json')

class SPARequestHandler(http.server.SimpleHTTPRequestHandler):
    """Custom handler for Single Page Application routing"""
    
    def do_GET(self):
        # Parse the URL
        parsed_path = urlparse(self.path)
        path = parsed_path.path
        
        # Handle API endpoints (mock data)
        if path.startswith('/api/'):
            self.handle_api_request(path, parsed_path.query)
            return
        
        # Handle static files
        if path.startswith('/assets/') or path.endswith(('.js', '.css', '.png', '.ico', '.wasm')):
            return super().do_GET()
        
        # For all other routes, serve index.html (SPA routing)
        if path != '/' and not os.path.exists('.' + path):
            self.path = '/index.html'
        
        return super().do_GET()
    
    def handle_api_request(self, path, query):
        """Handle mock API requests"""
        try:
            if path == '/api/jobs':
                # Mock jobs data
                jobs_data = {
                    "jobs": [
                        {
                            "id": 1,
                            "title": "Intern Position 1",
                            "company": "Pharmacy A",
                            "salary": "$39/hr",
                            "location": "1 Main St, Adelaide",
                            "distance": "16.1 km",
                            "date": "03/06/2025",
                            "time": "15am - 5pm",
                            "urgent": False
                        },
                        {
                            "id": 2,
                            "title": "Intern Position 2", 
                            "company": "Pharmacy B",
                            "salary": "$41/hr",
                            "location": "1 Main St, Adelaide",
                            "distance": "12.4 km",
                            "date": "03/06/2025",
                            "time": "13am - 7pm",
                            "urgent": False
                        },
                        {
                            "id": 3,
                            "title": "Pharmacist Position 3",
                            "company": "Pharmacy C", 
                            "salary": "$50/hr",
                            "location": "1 Main St, Adelaide",
                            "distance": "7.1 km",
                            "date": "03/06/2025",
                            "time": "12am - 7pm",
                            "urgent": False
                        },
                        {
                            "id": 4,
                            "title": "Student Position 4",
                            "company": "Pharmacy D",
                            "salary": "$35/hr", 
                            "location": "2 Main St, Adelaide",
                            "distance": "11.3 km",
                            "date": "03/06/2025",
                            "time": "16am - 7pm",
                            "urgent": False
                        },
                        {
                            "id": 5,
                            "title": "Student Position 5",
                            "company": "Pharmacy E",
                            "salary": "$42/hr",
                            "location": "5 km",
                            "distance": "10.9 km", 
                            "date": "03/06/2025",
                            "time": "16am - 5pm",
                            "urgent": True
                        }
                    ]
                }
                self.send_json_response(jobs_data)
                
            elif path == '/api/health':
                self.send_json_response({"status": "ok", "service": "loco-platform"})
                
            else:
                self.send_json_response({"error": "Not found"}, 404)
                
        except Exception as e:
            self.send_json_response({"error": str(e)}, 500)
    
    def send_json_response(self, data, status_code=200):
        """Send JSON response with proper headers"""
        json_data = json.dumps(data).encode('utf-8')
        
        self.send_response(status_code)
        self.send_header('Content-Type', 'application/json')
        self.send_header('Content-Length', str(len(json_data)))
        self.end_headers()
        self.wfile.write(json_data)
    
    def end_headers(self):
        # Add CORS headers for development
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS, PUT, DELETE')
        self.send_header('Access-Control-Allow-Headers', '*')
        
        # Security headers
        self.send_header('X-Content-Type-Options', 'nosniff')
        self.send_header('X-Frame-Options', 'DENY')
        self.send_header('X-XSS-Protection', '1; mode=block')
        
        # Ensure WASM files have correct MIME type
        if self.path.endswith('.wasm'):
            self.send_header('Content-Type', 'application/wasm')
        
        super().end_headers()

    def do_OPTIONS(self):
        """Handle CORS preflight requests"""
        self.send_response(200)
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS, PUT, DELETE')
        self.send_header('Access-Control-Allow-Headers', '*')
        self.end_headers()

if __name__ == "__main__":
    PORT = 8080
    
    # Change to the static directory
    static_dir = os.path.dirname(os.path.abspath(__file__))
    os.chdir(static_dir)
    
    with socketserver.TCPServer(("", PORT), SPARequestHandler) as httpd:
        print(f"🚀 Loco Platform SPA server running at http://localhost:{PORT}")
        print("✅ SPA routing configured") 
        print("✅ Mock API endpoints available")
        print("✅ CORS enabled for development")
        print("✅ WASM MIME types configured")
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\n🛑 Server stopped")