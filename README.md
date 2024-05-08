# ReverseSSH-API

ReverseSSH-API is a lightweight RESTful API designed to simplify the process of establishing reverse SSH connections. With ReverseSSH-API, users can easily access a list of available ports for remote access to their servers, enabling seamless accessibility to their services from anywhere. Simplify your SSH tunneling setup and streamline remote server access with ReverseSSH-API

## Features

- **Secure**: ReverseSSH-API uses a secure token-based authentication system to ensure that only authorized users can access the API.
- **Lightweight**: ReverseSSH-API is designed to be lightweight and easy to use, with a simple RESTful API that can be integrated into any application.
- **Flexible**: ReverseSSH-API provides a flexible way to manage reverse SSH connections, with support for multiple users and multiple ports.
- **Scalable**: ReverseSSH-API is built on a scalable architecture that can handle a large number of connections and users without compromising performance.

## Getting Started

To get started with ReverseSSH-API, follow these steps:

1. Clone the repository: `git clone https://github.com/ski-sync/api_reverse_proxy.git`
2. Set up environment variables:
   `cp .env.example .env`
   Copy code
   Edit the `.env` file with your desired configuration.
3. Run the Docker container: `docker compose up -d`
4. Access the API at `http://localhost:8000`

## API Reference

ReverseSSH-API provides a simple RESTful API with the following

- `GET api/ports`: Get a list of unused ports for reverse SSH connections.
- `GET api/register`: Register a list of ports for a user.
- `GET api/traefik`: get dynamic conf for traefik.

## License

ReverseSSH-API is licensed under the [MIT license](https://opensource.org/licenses/MIT).

## Contact

If you have any questions or feedback, feel free to contact us at [louis.sasse@protonmail.com](mailto:louis.sasse@protonmail.com).
