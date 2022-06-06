package lib

import (
	"testing"
)

func TestKeyValueMap(t *testing.T) {
	args := []string{"key1=value1", "key2=value2"}
	result, err := KeyValueMap(args)

	if err != nil {
		t.Fatal(err)
	}
	if result["key1"] != "value1" {
		t.Errorf("key1 failed, got %s", result["key1"])
	}
	if result["key2"] != "value2" {
		t.Errorf("key2 failed, got %s", result["key2"])
	}
}

func TestKeyValueMapError(t *testing.T) {

	// oops, don't have an = between key1 and value1
	args := []string{"key1value1", "key2=value2"}
	_, err := KeyValueMap(args)

	switch err.(type) {
	case BadKeyValueArg:
		t.Log("Got the correct error")
	default:
		t.Errorf("Expected to get BadKeyValueArg error, got %s", err)
	}
}
