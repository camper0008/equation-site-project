package db_impl

/*

import (
	"context"
	"equation-site-backend/models"
	"equation-site-backend/utils"
	"os"
	"time"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

type MongoDbSession struct {
	Token       string
	UserId      string `bson:"user_id"`
	DateCreated string `bson:"date_created"`
}

type MongoDb struct {
	client *mongo.Client
}

func (db *MongoDb) Init() {
	if os.Getenv("MONGODB_URI") == "" {
		panic("environment variable MONGODB_URI not set.")
	}

	ctx, cancel := context.WithTimeout(context.Background(), 20*time.Second)
	defer cancel()
	client, err := mongo.Connect(ctx, options.Client().ApplyURI(os.Getenv("MONGODB_URI")))
	if err != nil {
		panic(err)
	}

	db.client = client
}
func (db *MongoDb) InsertUser(user models.User) error {
	collection := db.client.Database("equation-site").Collection("users")
	_, err := collection.InsertOne(context.Background(), user)
	if err != nil {
		return err
	}
	return nil
}
func (db *MongoDb) UserFromUsername(username models.Username) (models.User, error) {
	collection := db.client.Database("equation-site").Collection("users")
	res := models.User{}
	err := collection.FindOne(context.Background(), bson.D{{Key: "username", Value: string(username)}}).Decode(res)
	if err != nil {
		return models.User{}, err
	}
	return res, nil

}
func (db *MongoDb) UserFromSession(session models.Session) (models.User, error) {
	sessionCollection := db.client.Database("equation-site").Collection("sessions")
	dbSession := MongoDbSession{}
	err := sessionCollection.FindOne(context.Background(), bson.D{{Key: "token", Value: string(session)}}).Decode(dbSession)
	if err != nil {
		return models.User{}, err
	}
	userCollection := db.client.Database("equation-site").Collection("users")
	res := models.User{}
	err = userCollection.FindOne(context.Background(), bson.D{{Key: "id", Value: string(dbSession.UserId)}}).Decode(res)
	if err != nil {
		return models.User{}, err
	}
	return res, nil
}
func (db *MongoDb) BindSessionToUser(session models.Session, user models.User) error {
	collection := db.client.Database("equation-site").Collection("sessions")
	_, err := collection.InsertOne(context.Background(), MongoDbSession{
		Token:       string(session),
		UserId:      string(user.Id),
		DateCreated: string(utils.TodayAsDateCreated()),
	})
	if err != nil {
		return err
	}
	return nil
}
func (db *MongoDb) UnbindSession(session models.Session) error {
	collection := db.client.Database("equation-site").Collection("sessions")
	_, err := collection.DeleteOne(context.Background(), bson.D{{Key: "token", Value: string(session)}})
	if err != nil {
		return err
	}
	return nil
}
func (db *MongoDb) InsertEquation(eq models.Equation) error {
	collection := db.client.Database("equation-site").Collection("equations")
	_, err := collection.InsertOne(context.Background(), eq)
	if err != nil {
		return err
	}
	return nil
}
func (db *MongoDb) EquationFromId(id models.Id) (models.Equation, error) {
	collection := db.client.Database("equation-site").Collection("equations")
	res := models.Equation{}
	err := collection.FindOne(context.Background(), bson.D{{Key: "id", Value: string(id)}}).Decode(res)
	if err != nil {
		return models.Equation{}, err
	}
	return res, nil
}
func (db *MongoDb) UpdateEquation(eq models.Equation) error {
	panic("unimplemented")
}
func (db *MongoDb) RemoveEquation(eq models.Equation) error {
	collection := db.client.Database("equation-site").Collection("equations")
	_, err := collection.DeleteOne(context.Background(), bson.D{{Key: "id", Value: string(eq.Id)}})
	if err != nil {
		return err
	}
	return nil
}
func (db *MongoDb) AllEquations() ([]models.PreviewableEquation, error) {
	cur, err := collection.Find(context.Background(), bson.D{})
	if err != nil {
		return err
	}
	defer cur.Close(context.Background())
	res := []models.PreviewableEquation{}
	if err = cur.All(context.Background(), &results); err != nil {
		return models.PreviewableEquation{}, nil
	}

	return res, nil
}
*/
