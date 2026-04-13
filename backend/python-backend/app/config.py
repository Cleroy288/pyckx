"""Application configuration — loads shared .env."""

from pathlib import Path

from pydantic_settings import BaseSettings

# Shared .env sits one level up (backend/.env)
ENV_PATH = Path(__file__).resolve().parents[2] / ".env"


class Settings(BaseSettings):
    """Settings shared with the Rust backend."""

    # Python-specific
    py_port: int = 8001
    py_host: str = "127.0.0.1"

    # Supabase (shared with Rust)
    sp_url: str = ""
    sp_anon: str = ""
    sp_service_role: str = ""

    # Redis (shared with Rust)
    redis_url: str = "redis://127.0.0.1:6379"

    model_config = {
        "env_file": str(ENV_PATH),
        "extra": "ignore",
    }


settings = Settings()
