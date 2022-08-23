package models

type Id string
type Session string
type UserId Id
type Username string
type Password string

type DB interface {
	UserFromUsername(Username) (User, error)
	UserFromSession(Session) (User, error)
	BindSessionToUser(Session, User) error
	UnbindSession(Session) error
}

type User struct {
	Id         UserId
	Username   Username
	Password   Password
	Permission Permission
}

type Permission int

const (
	UndefinedPermission = iota
	UserPermission
	ContributorPermission
	RootPermission
)
