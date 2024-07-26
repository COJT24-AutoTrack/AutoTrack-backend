# API Documentation

## Overview

This API provides endpoints for managing users, cars, tunings, maintenances, fuel efficiencies, accidents, and periodic inspections. Each resource has its own set of endpoints for CRUD operations. The base URL for all private endpoints is `http://your-domain.com`.

## Authentication

All private routes are protected by Firebase Authentication. Clients must include a valid JWT token in the Authorization header for these requests.

## Endpoints

### Test

- `GET /test`: Test the api endpoints. <strong>This endpoint does not require a bearer token.</strong>

  - Response:

    ```json
    {
      "message": "Hello World!"
    ```

### Users

- `POST /api/users`: Create a new user.
  - Request Body:

    ```json
    {
      "user_email": "user@example.com",
      "user_name": "John Doe",
      "user_password": "password123"
    }
    ```

  - Response:

    ```json
    {
      "user_id": 1,
      "user_email": "user@example.com",
      "user_name": "John Doe",
      "user_password": "password123",
      "created_at": "2023-06-21T10:20:30Z",
      "updated_at": "2023-06-21T10:20:30Z"
    }
    ```

- `GET /api/users`: Get all users.
  - Response:

    ```json

    [
      {
        "user_id": 1,
        "user_email": "user@example.com",
        "user_name": "John Doe",
        "user_password": "password123",
        "created_at": "2023-06-21T10:20:30Z",
        "updated_at": "2023-06-21T10:20:30Z"
      },
      ...
    ]
    ```

- `GET /api/users/:user_id`: Get a user by ID.
  - Path Parameters: `user_id` - User ID.
  - Response:

    ```json
    {
      "user_id": 1,
      "user_email": "user@example.com",
      "user_name": "John Doe",
      "user_password": "password123",
      "created_at": "2023-06-21T10:20:30Z",
      "updated_at": "2023-06-21T10:20:30Z"
    }
    ```

- `PUT /api/users/:user_id`: Update a user by ID.
  - Path Parameters: `user_id` - User ID.
  - Request Body:

    ```json
    {
      "user_email": "new_email@example.com",
      "user_name": "John Doe",
      "user_password": "newpassword123"
    }
    ```

  - Response:

    ```json
    {
      "user_id": 1,
      "user_email": "new_email@example.com",
      "user_name": "John Doe",
      "user_password": "newpassword123",
      "created_at": "2023-06-21T10:20:30Z",
      "updated_at": "2023-06-21T11:00:00Z"
    }
    ```

- `DELETE /api/users/:user_id`: Delete a user by ID.
  - Path Parameters: `user_id` - User ID.
  - Response: Status code indicating success or failure.

- `GET /api/users/:user_id/cars`: Get all cars associated with a user.
  - Path Parameters: `user_id` - User ID.
  - Response:

    ```json
    [
      {
        "car_id": 1,
        "car_name": "Toyota Prius",
        "carmodelnum": "X123",
        "car_color": "Blue",
        "car_mileage": 10000,
        "car_isflooding": false,
        "car_issmoked": false,
        "car_image_url": "http://example.com/image.jpg",
        "created_at": "2023-06-21T10:20:30Z",
        "updated_at": "2023-06-21T10:20:30Z"
      },
      {
        "car_id": 2,
        "car_name": "Honda Accord",
        "carmodelnum": "Y456",
        "car_color": "Red",
        "car_mileage": 20000,
        "car_isflooding": false,
        "car_issmoked": false,
        "car_image_url": "http://example.com/image2.jpg",
        "created_at": "2023-06-21T10:20:30Z",
        "updated_at": "2023-06-21T10:20:30Z"
      }
    ]
    ```

### Cars

- `POST /api/cars`: Create a new car.
  - Request Body:

    ```json
    {
      "car": {
        "car_name": "Toyota Prius",
        "carmodelnum": "X123",
        "car_color": "Blue",
        "car_mileage": 10000,
        "car_isflooding": false,
        "car_issmoked": false
      },
      "user_id": 1
    }
    ```

  - Response:

    ```json
    {
      "car_id": 1,
      "car_name": "Toyota Prius",
      "carmodelnum": "X123",
      "car_color": "Blue",
      "car_mileage": 10000,
      "car_isflooding": false,
      "car_issmoked": false,
      "created_at": "2023-06-21T10:20:30Z",
      "updated_at": "2023-06-21T10:20:30Z"
    }
    ```

- `GET /api/cars`: Get all cars.
  - Response:

    ```json
    [
      {
        "car_id": 1,
        "car_name": "Toyota Prius",
        "carmodelnum": "X123",
        "car_color": "Blue",
        "car_mileage": 10000,
        "car_isflooding": false,
        "car_issmoked": false,
        "created_at": "2023-06-21T10:20:30Z",
        "updated_at": "2023-06-21T10:20:30Z"
      },
      ...
    ]
    ```

- `GET /api/cars/:car_id`: Get a car by ID.
  - Path Parameters: `car_id` - Car ID.
  - Response:

    ```json
    {
      "car_id": 1,
      "car_name": "Toyota Prius",
      "carmodelnum": "X123",
      "car_color": "Blue",
      "car_mileage": 10000,
      "car_isflooding": false,
      "car_issmoked": false,
      "created_at": "2023-06-21T10:20:30Z",
      "updated_at": "2023-06-21T10:20:30Z"
    }
    ```

- `PUT /api/cars/:car_id`: Update a car by ID.
  - Path Parameters: `car_id` - Car ID.
  - Request Body:

    ```json
    {
      "car_name": "Toyota Prius",
      "carmodelnum": "X123",
      "car_color": "Red",
      "car_mileage": 15000,
      "car_isflooding": false,
      "car_issmoked": false
    }
    ```

  - Response:

    ```json
    {
      "car_id": 1,
      "car_name": "Toyota Prius",
      "carmodelnum": "X123",
      "car_color": "Red",
      "car_mileage": 15000,
      "car_isflooding": false,
      "car_issmoked": false,
      "created_at": "2023-06-21T10:20:30Z",
      "updated_at": "2023-06-21T11:00:00Z"
    }
    ```

- `DELETE /api/cars/:car_id`: Delete a car by ID.
  - Path Parameters: `car_id` - Car ID.
  - Response: Status code indicating success or failure.

- `PUT /api/cars/:car_id/image`: Update car image by ID.
  - Path Parameters: `car_id` - Car ID.
  - Request Body:

    ```json
    {
      "image_url": "https://example.com/path/to/car_image.jpg"
    }
    ```

  - Response:

    ```json
    {
      "car_id": 1,
      "car_name": "Toyota Prius",
      "carmodelnum": "X123",
      "car_color": "Blue",
      "car_mileage": 10000,
      "car_isflooding": false,
      "car_issmoked": false,
      "car_image_url": "https://example.com/path/to/car_image.jpg",
      "created_at": "2023-06-21T10:20:30Z",
      "updated_at": "2023-06-21T11:00:00Z"
    }
    ```

- `DELETE /api/cars/:car_id/image`: Delete car image by ID.
  - Path Parameters: `car_id` - Car ID.
  - Response: Status code indicating success or failure.

- `GET /api/cars/:car_id/tuning`: Get car tuning details by ID.
  - Path Parameters: `car_id` - Car ID.
  - Response:

    ```json
    [
      {
        "tuning_id": 1,
        "car_id": 1,
        "tuning_name": "Engine Tuning",
        "tuning_date": "2023-06-21",
        "tuning_description": "Detailed description of the tuning",
        "created_at": "2023-06-21T10:20:30Z",
        "updated_at": "2023-06-21T10:20:30Z"
      },
      {
        "tuning_id": 2,
        "car_id": 1,
        "tuning_name": "Suspension Tuning",
        "tuning_date": "2023-07-01",
        "tuning_description": "Detailed description of the tuning",
        "created_at": "2023-07-01T10:20:30Z",
        "updated_at": "2023-07-01T10:20:30Z"
      },
      ...
    ]
    ```

- `GET /api/cars/:car_id/maintenance`: Get car maintenance details by ID.
  - Path Parameters: `car_id` - Car ID.
  - Response:

    ```json
    [
      {
        "maint_id": 1,
        "car_id": 1,
        "maint_type": "Oil Change",
        "maint_date": "2023-06-21",
        "maint_description": "Changed engine oil",
        "created_at": "2023-06-21T10:20:30Z",
        "updated_at": "2023-06-21T10:20:30Z"
      },
      {
        "maint_id": 2,
        "car_id": 1,
        "maint_type": "Brake Inspection",
        "maint_date": "2023-07-01",
        "maint_description": "Inspected and replaced brake pads",
        "created_at": "2023-07-01T10:20:30Z",
        "updated_at": "2023-07-01T10:20:30Z"
      },
      ...
    ]
    ```

- `GET /api/cars/:car_id/fuel_efficiency`: Get car fuel efficiency details by ID.
  - Path Parameters: `car_id` - Car ID.
  - Response:

    ```json
    [
      {
        "fe_id": 1,
        "car_id": 1,
        "fe_date": "2023-06-21",
        "fe_amount": 40.5,
        "fe_unitprice": 1.2,
        "fe_mileage": 500,
        "created_at": "2023-06-21T10:20:30Z",
        "updated_at": "2023-06-21T10:20:30Z"
      },
      {
        "fe_id": 2,
        "car_id": 1,
        "fe_date": "2023-07-01",
        "fe_amount": 45.0,
        "fe_unitprice": 1.25,
        "fe_mileage": 550,
        "created_at": "2023-07-01T10:20:30Z",
        "updated_at": "2023-07-01T10:20:30Z"
      },
      ...
    ]
    ```

- `GET /api/cars/:car_id/fuel_efficiencies/calculate`: Calculate and get the fuel efficiency for a specific car.
  - Path Parameters: `car_id` - Car ID.
  - Response:

    ```json
    {
      "car_id": 1,
      "total_fuel_efficiency": 8.6,
      "fuel_efficiencies": [
        {
          "fe_id": 1,
          "fuel_efficiency": 8.6
        },
        {
          "fe_id": 2,
          "fuel_efficiency": 9.0
        },
        ...
      ]
    }
    ```


### Tunings

- `POST /api/tunings`: Create a new tuning.
  - Request Body:

    ```json
    {
      "car_id": 1,
      "tuning_name": "Engine Overhaul",
      "tuning_date": "2023-06-20",
      "tuning_description": "Complete engine overhaul."
    }
    ```

  - Response:

    ```json
    {
      "tuning_id": 1,
      "car_id": 1,
      "tuning_name": "Engine Overhaul",
      "tuning_date": "2023-06-20",
      "tuning_description": "Complete engine overhaul.",
      "created_at": "2023-06-21T10:20:30Z",
      "updated_at": "2023-06-21T10:20:30Z"
    }
    ```

- `GET /api/tunings`: Get all tunings.
  - Response:

    ```json
    [
      {
        "tuning_id": 1,
        "car_id": 1,
        "tuning_name": "Engine Overhaul",
        "tuning_date": "2023-06-20",
        "tuning_description": "Complete engine overhaul.",
        "created_at": "2023-06-21T10:20:30Z",
        "updated_at": "2023-06-21T10:20:30Z"
      },
      ...
    ]
    ```

- `GET /api/tunings/:tuning_id`: Get a tuning by ID.
  - Path Parameters: `tuning_id` - Tuning ID.
  - Response:

    ```json
    {
      "tuning_id": 1,
      "car_id": 1,
      "tuning_name": "Engine Overhaul",
      "tuning_date": "2023-06-20",
      "tuning_description": "Complete engine overhaul.",
      "created_at": "2023-06-21T10:20:30Z",
      "updated_at": "2023-06-21T10:20:30Z"
    }
    ```

- `PUT /api/tunings/:tuning_id`: Update a tuning by ID.
  - Path Parameters: `tuning_id` - Tuning ID.
  - Request Body:

    ```json
    {
      "car_id": 1,
      "tuning_name": "Engine Overhaul",
      "tuning_date": "2023-06-20",
      "tuning_description": "Complete engine overhaul with new parts."
    }
    ```

  - Response:

    ```json
    {
      "tuning_id": 1,
      "car_id": 1,
      "tuning_name": "Engine Overhaul",
      "tuning_date": "2023-06-20",
      "tuning_description": "Complete engine overhaul with new parts.",
      "created_at": "2023-06-21T10:20:30Z",
      "updated_at": "2023-06-21T11:00:00Z"
    }
    ```

- `DELETE /api/tunings/:tuning_id`: Delete a tuning by ID.
  - Path Parameters: `tuning_id` - Tuning ID.
  - Response: Status code indicating success or failure.

### Maintenances

- `POST /api/maintenances`: Create a new maintenance.
  - Request Body:

    ```json
    {
      "car_id": 1,
      "maint_type": "Oil Change",
      "maint_date": "2023-06-20",
      "maint_description": "Changed the oil."
    }
    ```

  - Response:

    ```json
    {
      "maint_id": 1,
      "car_id": 1,
      "maint_type": "Oil Change",
      "maint_date": "2023-06-20",
      "maint_description": "Changed the oil.",
      "created_at": "2023-06-21T10:20:30Z",
      "updated_at": "2023-06-21T10:20:30Z"
    }
    ```

- `GET /api/maintenances`: Get all maintenances.
  - Response:

    ```json
    [
      {
        "maint_id": 1,
        "car_id": 1,
        "maint_type": "Oil Change",
        "maint_date": "2023-06-20",
        "maint_description": "Changed the oil.",
        "created_at": "2023-06-21T10:20:30Z",
        "updated_at": "2023-06-21T10:20:30Z"
      },
      ...
    ]
    ```

- `GET /api/maintenances/:maint_id`: Get a maintenance by ID.
  - Path Parameters: `maint_id` - Maintenance ID.
  - Response:

    ```json
    {
      "maint_id": 1,
      "car_id": 1,
      "maint_type": "Oil Change",
      "maint_date": "2023-06-20",
      "maint_description": "Changed the oil.",
      "created_at": "2023-06-21T10:20:30Z",
      "updated_at": "2023-06-21T10:20:30Z"
    }
    ```

- `PUT /api/maintenances/:maint_id`: Update a maintenance by ID.
  - Path Parameters: `maint_id` - Maintenance ID.
  - Request Body:

    ```json
    {
      "car_id": 1,
      "maint_type": "Oil Change",
      "maint_date": "2023-06-20",
      "maint_description": "Changed the oil and filter."
    }
    ```

  - Response:

    ```json
    {
      "maint_id": 1,
      "car_id": 1,
      "maint_type": "Oil Change",
      "maint_date": "2023-06-20",
      "maint_description": "Changed the oil and filter.",
      "created_at": "2023-06-21T10:20:30Z",
      "updated_at": "2023-06-21T11:00:00Z"
    }
    ```

- `DELETE /api/maintenances/:maint_id`: Delete a maintenance by ID.
  - Path Parameters: `maint_id` - Maintenance ID.
  - Response: Status code indicating success or failure.

### Fuel Efficiencies

- `POST /api/fuel_efficiencies`: Create a new fuel efficiency record.
  - Request Body:

    ```json
    {
      "car_id": 1,
      "fe_date": "2023-06-20",
      "fe_amount": 40.5,
      "fe_unitprice": 1.5,
      "fe_mileage": 350
    }
    ```

  - Response:

    ```json
    {
      "fe_id": 1,
      "car_id": 1,
      "fe_date": "2023-06-20",
      "fe_amount": 40.5,
      "fe_unitprice": 1.5,
      "fe_mileage": 350,
      "created_at": "2023-06-21T10:20:30Z",
      "updated_at": "2023-06-21T10:20:30Z"
    }
    ```

- `GET /api/fuel_efficiencies`: Get all fuel efficiency records.
  - Response:

    ```json
    [
      {
        "fe_id": 1,
        "car_id": 1,
        "fe_date": "2023-06-20",
        "fe_amount": 40.5,
        "fe_unitprice": 1.5,
        "fe_mileage": 350,
        "created_at": "2023-06-21T10:20:30Z",
        "updated_at": "2023-06-21T10:20:30Z"
      },
      ...
    ]
    ```

- `GET /api/fuel_efficiencies/:fe_id`: Get a fuel efficiency record by ID.
  - Path Parameters: `fe_id` - Fuel Efficiency ID.
  - Response:

    ```json
    {
      "fe_id": 1,
      "car_id": 1,
      "fe_date": "2023-06-20",
      "fe_amount": 40.5,
      "fe_unitprice": 1.5,
      "fe_mileage": 350,
      "created_at": "2023-06-21T10:20:30Z",
      "updated_at": "2023-06-21T10:20:30Z"
    }
    ```

- `PUT /api/fuel_efficiencies/:fe_id`: Update a fuel efficiency record by ID.
  - Path Parameters: `fe_id` - Fuel Efficiency ID.
  - Request Body:

    ```json
    {
      "car_id": 1,
      "fe_date": "2023-06-20",
      "fe_amount": 45.0,
      "fe_unitprice": 1.6,
      "fe_mileage": 360
    }
    ```

  - Response:

    ```json
    {
      "fe_id": 1,
      "car_id": 1,
      "fe_date": "2023-06-20",
      "fe_amount": 45.0,
      "fe_unitprice": 1.6,
      "fe_mileage": 360,
      "created_at": "2023-06-21T10:20:30Z",
      "updated_at": "2023-06-21T11:00:00Z"
    }
    ```

- `DELETE /api/fuel_efficiencies/:fe_id`: Delete a fuel efficiency record by ID.
  - Path Parameters: `fe_id` - Fuel Efficiency ID.
  - Response: Status code indicating success or failure.

### Accidents

- `POST /api/accidents`: Create a new accident record.
  - Request Body:

    ```json
    {
      "car_id": 1,
      "accident_date": "2023-06-20",
      "accident_description": "Minor collision."
    }
    ```

  - Response:

    ```json
    {
      "accident_id": 1,
      "car_id": 1,
      "accident_date": "2023-06-20",
      "accident_description": "Minor collision.",
      "created_at": "2023-06-21T10:20:30Z",
      "updated_at": "2023-06-21T10:20:30Z"
    }
    ```

- `GET /api/accidents`: Get all accident records.
  - Response:

    ```json
    [
      {
        "accident_id": 1,
        "car_id": 1,
        "accident_date": "2023-06-20",
        "accident_description": "Minor collision.",
        "created_at": "2023-06-21T10:20:30Z",
        "updated_at": "2023-06-21T10:20:30Z"
      },
      ...
    ]
    ```

- `GET /api/accidents/:accident_id`: Get an accident record by ID.
  - Path Parameters: `accident_id` - Accident ID.
  - Response:

    ```json
    {
      "accident_id": 1,
      "car_id": 1,
      "accident_date": "2023-06-20",
      "accident_description": "Minor collision.",
      "created_at": "2023-06-21T10:20:30Z",
      "updated_at": "2023-06-21T10:20:30Z"
    }
    ```

- `PUT /api/accidents/:accident_id`: Update an accident record by ID.
  - Path Parameters: `accident_id` - Accident ID.
  - Request Body:

    ```json
    {
      "car_id": 1,
      "accident_date": "2023-06-20",
      "accident_description": "Minor collision with repair."
    }
    ```

  - Response:

    ```json
    {
      "accident_id": 1,
      "car_id": 1,
      "accident_date": "2023-06-20",
      "accident_description": "Minor collision with repair.",
      "created_at": "2023-06-21T10:20:30Z",
      "updated_at": "2023-06-21T11:00:00Z"
    }
    ```

- `DELETE /api/accidents/:accident_id`: Delete an accident record by ID.
  - Path Parameters: `accident_id` - Accident ID.
  - Response: Status code indicating success or failure.

### Periodic Inspections

- `POST /api/periodic_inspections`: Create a new periodic inspection record.
  - Request Body:

    ```json
    {
      "car_id": 1,
      "pi_name": "Annual Inspection",
      "pi_date": "2023-06-20",
      "pi_nextdate": "2024-06-20"
    }
    ```

  - Response:

    ```json
    {
      "pi_id": 1,
      "car_id": 1,
      "pi_name": "Annual Inspection",
      "pi_date": "2023-06-20",
      "pi_nextdate": "2024-06-20",
      "created_at": "2023-06-21T10:20:30Z",
      "updated_at": "2023-06-21T10:20:30Z"
    }
    ```

- `GET /api/periodic_inspections`: Get all periodic inspection records.
  - Response:

    ```json
    [
      {
        "pi_id": 1,
        "car_id": 1,
        "pi_name": "Annual Inspection",
        "pi_date": "2023-06-20",
        "pi_nextdate": "2024-06-20",
        "created_at": "2023-06-21T10:20:30Z",
        "updated_at": "2023-06-21T10:20:30Z"
      },
      ...
    ]
    ```

- `GET /api/periodic_inspections/:pi_id`: Get a periodic inspection record by ID.
  - Path Parameters: `pi_id` - Periodic Inspection ID.
  - Response:

    ```json
    {
      "pi_id": 1,
      "car_id": 1,
      "pi_name": "Annual Inspection",
      "pi_date": "2023-06-20",
      "pi_nextdate": "2024-06-20",
      "created_at": "2023-06-21T10:20:30Z",
      "updated_at": "2023-06-21T10:20:30Z"
    }
    ```

- `PUT /api/periodic_inspections/:pi_id`: Update a periodic inspection record by ID.
  - Path Parameters: `pi_id` - Periodic Inspection ID.
  - Request Body:

    ```json
    {
      "car_id": 1,
      "pi_name": "Annual Inspection",
      "pi_date": "2023-06-20",
      "pi_nextdate": "2024-06-20"
    }
    ```

  - Response:

    ```json
    {
      "pi_id": 1,
      "car_id": 1,
      "pi_name": "Annual Inspection",
      "pi_date": "2023-06-20",
      "pi_nextdate": "2024-06-20",
      "created_at": "2023-06-21T10:20:30Z",
      "updated_at": "2023-06-21T11:00:00Z"
    }
    ```

- `DELETE /api/periodic_inspections/:pi_id`: Delete a periodic inspection record by ID.
  - Path Parameters: `pi_id` - Periodic Inspection ID.
  - Response: Status code indicating success or failure.

### Images

- `POST /images`: Upload a new image.
  - Request: Multipart form data with the image file.
  - example

    ```sh
    curl -X POST http://localhost:8369/images -F "file=@{pathOfFile}"
    ```

## Models

### User

```json
{
  "user_id": "Optional<Integer>",
  "user_email": "String",
  "user_name": "String",
  "user_password": "String",
  "created_at": "Optional<OffsetDateTime>",
  "updated_at": "Optional<OffsetDateTime>"
}
```

### Car

```json
{
  "car_id": "Optional<Integer>",
  "car_name": "String",
  "carmodelnum": "String",
  "car_color": "String",
  "car_mileage": "Integer",
  "car_isflooding": "Boolean",
  "car_issmoked": "Boolean",
  "created_at": "Optional<OffsetDateTime>",
  "updated_at": "Optional<OffsetDateTime>"
}
```

### Tuning

```json
{
  "tuning_id": "Optional<Integer>",
  "car_id": "Integer",
  "tuning_name": "String",
  "tuning_date": "Date",
  "tuning_description": "String",
  "created_at": "Optional<OffsetDateTime>",
  "updated_at": "Optional<OffsetDateTime>"
}
```

### Maintenance

```json
{
  "maint_id": "Optional<Integer>",
  "car_id": "Integer",
  "maint_type": "String",
  "maint_date": "Date",
  "maint_description": "String",
  "created_at": "Optional<OffsetDateTime>",
  "updated_at": "Optional<OffsetDateTime>"
}
```

### FuelEfficiency

```json
{
  "fe_id": "Optional<Integer>",
  "car_id": "Integer",
  "fe_date": "Date",
  "fe_amount": "Float",
  "fe_unitprice": "Float",
  "fe_mileage": "Integer",
  "created_at": "Optional<OffsetDateTime>",
  "updated_at": "Optional<OffsetDateTime>"
}
```

### Accident

```json
{
  "accident_id": "Optional<Integer>",
  "car_id": "Integer",
  "accident_date": "Date",
  "accident_description": "String",
  "created_at": "Optional<OffsetDateTime>",
  "updated_at": "Optional<OffsetDateTime>"
}
```

### PeriodicInspection

```json
{
  "pi_id": "Optional<Integer>",
  "car_id": "Integer",
  "pi_name": "String",
  "pi_date": "Date",
  "pi_nextdate": "Date",
  "created_at": "Optional<OffsetDateTime>",
  "updated_at": "Optional<OffsetDateTime>"
}
```

### UserCar

```json
{
  "user_id": "Integer",
  "car_id": "Integer"
}
```

### CreateCarRequest

```json
{
  "car": "Car",
  "user_id": "Integer"
}
```

## Error Handling

- 500 Internal Server Error: The server encountered an error processing the request.
- 404 Not Found: The requested resource was not found.
- 201 Created: The resource was successfully created.
- 200 OK: The request was successful.

## Conclusion

This API provides comprehensive endpoints to manage users, cars, tunings, maintenances, fuel efficiencies, accidents, and periodic inspections. Use this documentation to interact with the API effectively. Ensure that the necessary security measures are in place before deploying the API in a production environment.
