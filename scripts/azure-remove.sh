#!/bin/bash

##### WARNING
#
# ./scripts/remove-azure.sh <resource-group-name> <name-prefix>
#
# Will remove anything which matches <name-prefix> !!!
#
#####

AZURE_RESOURCE_GROUP=$1
NAME_PREFIX=$2
RESOURCE_LIST=$(az resource list -g "$AZURE_RESOURCE_GROUP")
echo "**** INFO RESOURCE_LIST ****"
echo "$RESOURCE_LIST"
echo "$RESOURCE_LIST" > azure_resource_list.json
echo "****************************"

# Delete resources in a specific order, as dependency on one another might prevent resource deletion
RESOURCE_TYPES=(
    "Microsoft.Compute/virtualMachines"
    "Microsoft.Network/networkInterfaces"
    "Microsoft.Network/networkSecurityGroups"
    "Microsoft.Network/publicIPAddresses"
    "Microsoft.Network/virtualNetworks"
    "Microsoft.Compute/images"
    "Microsoft.Compute/disks"
)

for RESOURCE_TYPE in "${RESOURCE_TYPES[@]}"; do
    echo -e "\nDeleting $RESOURCE_TYPE\n"
    FILTERED_RESOURCES=$(echo "$RESOURCE_LIST" | jq -r "map(select(.type == \"$RESOURCE_TYPE\") | select(.name|test(\"$NAME_PREFIX\")))")
    FILTERED_RESOURCES_LEN=$(echo "$FILTERED_RESOURCES" | jq -r "length")
    for i in $(seq 0 $(("$FILTERED_RESOURCES_LEN" - 1))); do
        RESOURCE_ID=$(echo "$FILTERED_RESOURCES" | jq -r ".[$i].id")
        echo "Resource $RESOURCE_ID will be deleted"
        az resource delete --ids "$RESOURCE_ID"
    done
done
