package db_impl

import (
	"equation-site-backend/models"
	"fmt"

	"golang.org/x/crypto/bcrypt"
)

type MemoryDb struct {
	users    []models.User
	sessions map[models.Session]models.User
}

func (db *MemoryDb) Init() {
	if pass, err := bcrypt.GenerateFromPassword([]byte("pass"), bcrypt.DefaultCost); err == nil {
		db.users = []models.User{
			models.User{
				Username:   models.Username("user"),
				Password:   models.Password(pass),
				Permission: models.ContributorPermission,
			},
		}
		db.sessions = make(map[models.Session]models.User)
	}
}

func (db *MemoryDb) UserFromUsername(username models.Username) (models.User, error) {
	for _, user := range db.users {
		if user.Username == username {
			return user, nil
		}
	}
	return models.User{}, fmt.Errorf("user does not exist")
}

func (db *MemoryDb) UserFromSession(session models.Session) (models.User, error) {
	for userSession, user := range db.sessions {
		if userSession == session {
			return user, nil
		}
	}
	return models.User{}, fmt.Errorf("session does not exist")
}

func (db *MemoryDb) BindSessionToUser(session models.Session, user models.User) error {
	if _, found := db.sessions[session]; found != false {
		return fmt.Errorf("session collision")
	}
	db.sessions[session] = user
	return nil
}

func (db *MemoryDb) UnbindSession(session models.Session) error {
	if _, found := db.sessions[session]; found == false {
		return fmt.Errorf("session not bound")
	}
	delete(db.sessions, session)
	return nil
}
