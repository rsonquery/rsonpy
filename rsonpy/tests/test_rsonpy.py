import os
import pytest
import rsonpy

# Path to the test JSON file
TEST_JSON_FILE = os.path.join(os.path.dirname(__file__), "data", "test.json")


def test_loads_names():
    """Test rsonpy.loads with string input"""
    with open(TEST_JSON_FILE) as f:
        s = f.read()

    # Query all user names
    results = list(rsonpy.loads(s, "$.users[*].name"))
    expected = ["Alice", "Bob", "Charlie"]
    assert results == expected


def test_load_ids():
    """Test rsonpy.load with file input"""
    results = list(rsonpy.load(TEST_JSON_FILE, "$.users[*].id"))
    expected = [1, 2, 3]
    assert results == expected


def test_load_roles():
    """Test nested array query"""
    with open(TEST_JSON_FILE) as f:
        s = f.read()

    results = list(rsonpy.loads(s, "$.users[*].roles[*]"))
    # Flattened list of all roles
    expected = ["admin", "editor", "viewer", "editor", "viewer"]
    assert results == expected


def test_load_file_not_found():
    """Test that FileNotFoundError is raised for missing file"""
    with pytest.raises(FileNotFoundError):
        list(rsonpy.load("nonexistent.json", "$.users[*].id"))
