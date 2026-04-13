"""Health check route."""

from fastapi import APIRouter

router = APIRouter()


@router.get("/health")
async def health_check() -> dict:
    """Return service status."""
    return {"status": "ok", "service": "python-backend"}
