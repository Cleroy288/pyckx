"""Health check endpoint tests."""

import pytest
from httpx import ASGITransport, AsyncClient

from app.main import app


@pytest.mark.asyncio
async def test_health_check_returns_ok():
    """Health endpoint returns status ok."""
    transport = ASGITransport(app=app)
    async with AsyncClient(
        transport=transport,
        base_url="http://test",
    ) as client:
        response = await client.get("/py-api/health")

    assert response.status_code == 200
    assert response.json()["status"] == "ok"
