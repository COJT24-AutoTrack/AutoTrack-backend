# API Documentation

## Overview

This API provides endpoints for managing users, cars, tunings, maintenances, fuel efficiencies, accidents, and periodic inspections. Each resource has its own set of endpoints for CRUD operations. The base URL for all endpoints is ~`http://your-domain.com`~.

## Authentication

Currently, this API does not include authentication. Ensure that your API server is secure if deployed in a production environment.

## Endpoints

### Root

- `GET /`: Returns a "Hello, world!" message.
- `POST /`: Returns a "Hello, world!" message.

### Users

- `POST /users`: Create a new user.
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

- `GET /users`: Get all users.
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

- `GET /users/:id`: Get a user by ID.
  - Path Parameters: `id` - User ID.
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

- `PUT /users/:id`: Update a user by ID.
  - Path Parameters: `id` - User ID.
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

- `DELETE /users/:id`: Delete a user by ID.
  - Path Parameters: `id` - User ID.
  - Response: Status code indicating success or failure.

### Cars

- `POST /cars`: Create a new car.
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

- `GET /cars`: Get all cars.
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

- `GET /cars/:id`: Get a car by ID.
  - Path Parameters: `id` - Car ID.
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

- `PUT /cars/:id`: Update a car by ID.
  - Path Parameters: `id` - Car ID.
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

- `DELETE /cars/:id`: Delete a car by ID.
  - Path Parameters: `id` - Car ID.
  - Response: Status code indicating success or failure.

- `PUT /cars/:id/image`: Update car image by ID.
  - Path Parameters: `id` - Car ID.
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

### Tunings

- `POST /tunings`: Create a new tuning.
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

- `GET /tunings`: Get all tunings.
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

- `GET /tunings/:id`: Get a tuning by ID.
  - Path Parameters: `id` - Tuning ID.
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

- `PUT /tunings/:id`: Update a tuning by ID.
  - Path Parameters: `id` - Tuning ID.
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

- `DELETE /tunings/:id`: Delete a tuning by ID.
  - Path Parameters: `id` - Tuning ID.
  - Response: Status code indicating success or failure.

### Maintenances

- `POST /maintenances`: Create a new maintenance.
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

- `GET /maintenances`: Get all maintenances.
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

- `GET /maintenances/:id`: Get a maintenance by ID.
  - Path Parameters: `id` - Maintenance ID.
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

- `PUT /maintenances/:id`: Update a maintenance by ID.
  - Path Parameters: `id` - Maintenance ID.
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

- `DELETE /maintenances/:id`: Delete a maintenance by ID.
  - Path Parameters: `id` - Maintenance ID.
  - Response: Status code indicating success or failure.

### Fuel Efficiencies

- `POST /fuel_efficiencies`: Create a new fuel efficiency record.
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

- `GET /fuel_efficiencies`: Get all fuel efficiency records.
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

- `GET /fuel_efficiencies/:id`: Get a fuel efficiency record by ID.
  - Path Parameters: `id` - Fuel Efficiency ID.
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

- `PUT /fuel_efficiencies/:id`: Update a fuel efficiency record by ID.
  - Path Parameters: `id` - Fuel Efficiency ID.
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

- `DELETE /fuel_efficiencies/:id`: Delete a fuel efficiency record by ID.
  - Path Parameters: `id` - Fuel Efficiency ID.
  - Response: Status code indicating success or failure.

### Accidents

- `POST /accidents`: Create a new accident record.
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

- `GET /accidents`: Get all accident records.
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

- `GET /accidents/:id`: Get an accident record by ID.
  - Path Parameters: `id` - Accident ID.
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

- `PUT /accidents/:id`: Update an accident record by ID.
  - Path Parameters: `id` - Accident ID.
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

- `DELETE /accidents/:id`: Delete an accident record by ID.
  - Path Parameters: `id` - Accident ID.
  - Response: Status code indicating success or failure.

### Periodic Inspections

- `POST /periodic_inspections`: Create a new periodic inspection record.
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

- `GET /periodic_inspections`: Get all periodic inspection records.
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

- `GET /periodic_inspections/:id`: Get a periodic inspection record by ID.
  - Path Parameters: `id` - Periodic Inspection ID.
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

- `PUT /periodic_inspections/:id`: Update a periodic inspection record by ID.
  - Path Parameters: `id` - Periodic Inspection ID.
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

- `DELETE /periodic_inspections/:id`: Delete a periodic inspection record by ID.
  - Path Parameters: `id` - Periodic Inspection ID.
  - Response: Status code indicating success or failure.

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
