package utils

import (
	"crypto/rand"
	"encoding/hex"
	"equation-site-backend/models"
	"os"
	"time"
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

func TodayAsDateCreated() models.DateCreated {
	today := time.Now()
	return models.DateCreated(today.Format(time.RFC3339))
}
