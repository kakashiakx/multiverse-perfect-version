import os
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path


ROOT = Path(__file__).resolve().parent
HOST = os.environ.get("HOST", "0.0.0.0")
PORT = int(os.environ.get("PORT", "8000"))


class OmegaPrimeHandler(SimpleHTTPRequestHandler):
    """Serve the OMEGA-PRIME dashboard and repository assets."""

    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=str(ROOT), **kwargs)


def main() -> None:
    server = ThreadingHTTPServer((HOST, PORT), OmegaPrimeHandler)
    print(f"OMEGA-PRIME đang chạy tại http://{HOST}:{PORT}")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\nĐang dừng máy chủ...")
    finally:
        server.server_close()


if __name__ == "__main__":
    main()
