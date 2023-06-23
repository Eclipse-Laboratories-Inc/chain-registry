# Eclipse Chain Registry API Documentation

## Introduction

Eclipse Chain Registry is an API built with Rust and the Rocket web framework. It provides a means of managing both EVM (Ethereum Virtual Machine) and SVM (Solana Virtual Machine) blockchain chains, including their associated metadata. This API is built on a set of CRUD operations that allow users to add, view, update, and delete chain information.

## Setup

To begin using the API, first ensure that you have set the `API_KEY` environment variable. This key is required to authorize certain actions, such as adding and removing chain records. 

## API Overview

This API has endpoints that fall under two categories: `evm_chains` and `svm_chains`. Each category contains endpoints for creating, reading, updating, and deleting records.

- GET `/evm_chains`: Fetches all EVM chains.
- GET `/evm_chains/<slug>`: Fetches a single EVM chain by its slug.
- POST `/evm_chains`: Creates a new EVM chain.
- PATCH `/evm_chains/<chain_id>`: Updates an existing EVM chain.
- DELETE `/evm_chains/<chain_id>`: Deletes an existing EVM chain.

The same routes and operations are available for `svm_chains`:

- GET `/svm_chains`: Fetches all SVM chains.
- POST `/svm_chains`: Creates a new SVM chain.
- DELETE `/svm_chains/<chain_name>`: Deletes an existing SVM chain.

Additionally, the API includes a `/health` endpoint for health checks.

## Using the API with the `curlie` CLI Tool

Curlie is a powerful command-line HTTP client. It is a hybrid of cURL and HTTPie, providing a pleasant interface and rich functionality.

For instance, to fetch all EVM chains, you could use the following command:

```bash
curlie GET https://api.chains.eclipse.builders/evm_chains
```

To fetch a specific EVM chain by its slug:

```bash
curlie GET https://api.chains.eclipse.builders/evm_chains/<slug>
```

To delete a specific SVM chain (note that this operation requires the `x-api-key` header for authentication):

```bash
curlie DELETE https://api.chains.eclipse.builders/svm_chains/<chain_name> x-api-key:<Your-API-Key>
```

To create a new EVM chain:

```bash
curlie POST https://api.chains.eclipse.builders/evm_chains x-api-key:<Your-API-Key> Content-Type:application/json < JSON-data
```

In the above command, `JSON-data` should contain the necessary data for the new chain, such as the `chain_id`, `rpc_urls`, `block_explorer_urls`, etc.

Note: Replace `https://your-api-url` with your actual API URL.

## Updating an EVM Chain (PATCH Request)

The PATCH endpoint (`/evm_chains/<chain_id>`) provides a way to update an existing EVM chain's data. With this endpoint, you can modify details of a specific EVM chain identified by its `chain_id`. 

A PATCH request requires the `x-api-key` header for authentication, ensuring that only authorized users can update chain information.

The body of the PATCH request should contain a JSON object with the fields you want to update. The fields could include any of the following:

- `chain_id`: A string representing the unique chain ID.
- `rpc_urls`: An array of strings containing RPC URLs for the chain.
- `block_explorer_urls`: An array of strings containing block explorer URLs for the chain.
- `icon_urls`: An array of strings containing URLs of icons representing the chain.
- `chain_name`: A string representing the chain name.
- `native_currency_name`: A string representing the native currency's name.
- `native_currency_decimals`: An integer representing the number of decimal places in the native currency.
- `native_currency_symbol`: A string representing the native currency's symbol.
- `data_availability`: A string indicating the data availability status.
- `slug`: A string representing a URL-friendly version of the chain name.

The PATCH request will apply the provided updates to the specified chain, and it only modifies the fields you specify in the request body. Fields that you do not specify will remain unchanged.

Here's how you might use the `curlie` command-line tool to send a PATCH request:

```bash
curlie PATCH https://api.chains.eclipse.builders/evm_chains/<chain_id> x-api-key:<Your-API-Key> Content-Type:application/json '{"native_currency_name": "New Currency Name", "native_currency_symbol": "NCS"}'
```

In this example, we're updating the `native_currency_name` and `native_currency_symbol` fields of the EVM chain with a specified `chain_id`. Replace `<chain_id>` with the actual ID of the chain you want to update, and `<Your-API-Key>` with your actual API key.

The API will respond with the HTTP status code `OK` if the update was successful. If the specified chain is not found, it will respond with the `NotFound` status code. If an error occurs during the update operation, it will respond with the `InternalServerError` status code.

## Security

Security is ensured through the use of API keys. The `x-api-key` header must be provided when creating, updating, or deleting chain records. Unauthorized requests will be rejected with a `BadRequest` HTTP status.

## Conclusion

This documentation provides an overview of the Eclipse Chain Registry API. The API offers a set of CRUD operations for managing blockchain chain data, with `curlie` used to interact with the API from the command line.