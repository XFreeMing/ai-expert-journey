"""Shared test fixtures and configuration."""

import sys
from pathlib import Path

import pytest

# Add project root and shared to path
ROOT = Path(__file__).parent.parent
sys.path.insert(0, str(ROOT))
sys.path.insert(0, str(ROOT / "shared"))


@pytest.fixture
def project_root():
    return ROOT
