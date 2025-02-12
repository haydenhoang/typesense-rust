#!/bin/sh

openapi-generator-cli  generate -i "openapi.yml" -g rust -o "./typesense_codegen"

find "./typesense_codegen/src/apis" -type f -name "*.rs" -exec sed -i 's/local_var_client = \&local_var_configuration.client;/local_var_client = \&mut local_var_configuration.client;/g' {} +
find "./typesense_codegen/src/apis" -type f -name "*.rs" -exec sed -i 's/configuration: \&configuration::Configuration,/configuration: \&mut configuration::Configuration,/g' {} +
