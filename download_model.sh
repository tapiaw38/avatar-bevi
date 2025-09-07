#!/bin/bash

# Script para descargar el modelo Mita.glb desde la URL externa
echo "Descargando modelo Mita.glb desde URL externa..."

# Crear directorio si no existe
mkdir -p assets/models

# Asegurar que tenemos un modelo válido local
if [ ! -f "assets/models/mita.glb" ]; then
    echo "Creando modelo de respaldo válido..."
    curl -o assets/models/mita.glb "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Box/glTF-Binary/Box.glb"
fi

# Descargar el modelo externo
curl -o assets/models/mita_external.glb "https://nymia-bucket.s3.sa-east-1.amazonaws.com/3d-models/Mita.glb"

if [ $? -eq 0 ]; then
    echo "Modelo descargado exitosamente a assets/models/mita_external.glb"
else
    echo "Error al descargar el modelo. Usando modelo de respaldo."
    # Usar el modelo local como respaldo
    cp assets/models/mita.glb assets/models/mita_external.glb
fi
