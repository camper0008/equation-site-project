package utils

import (
	"crypto/rand"
	"encoding/hex"
	"os"
)

func GenerateSecureToken(length int) (string, error) {
	b := make([]byte, length)
	if _, err := rand.Read(b); err != nil {
		return "", err
	}
	return hex.EncodeToString(b), nil
}

func Domain() string {
	return os.Getenv("DOMAIN")
}
