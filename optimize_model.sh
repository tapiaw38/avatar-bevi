#!/bin/bash

# Script para optimizar el modelo Mita.glb
echo "Optimizando modelo Mita.glb..."

# Verificar si gltf-pipeline está instalado
if ! command -v gltf-pipeline &> /dev/null; then
    echo "gltf-pipeline no está instalado. Instalando..."
    npm install -g gltf-pipeline
fi

# Crear una versión optimizada del modelo
if [ -f "assets/models/mita_external.glb" ]; then
    echo "Creando versión optimizada del modelo..."
    gltf-pipeline -i assets/models/mita_external.glb -o assets/models/mita_optimized.glb --draco.compressionLevel 7 --draco.quantizePositionBits 11 --draco.quantizeNormalBits 8 --draco.quantizeTexcoordBits 10
    
    if [ $? -eq 0 ]; then
        echo "Modelo optimizado creado exitosamente"
        # Reemplazar el modelo original con el optimizado
        mv assets/models/mita_optimized.glb assets/models/mita.glb
        echo "Modelo optimizado reemplazado como mita.glb"
    else
        echo "Error al optimizar el modelo. Usando modelo de respaldo."
        # Usar el modelo de respaldo que ya funciona
        cp assets/models/mita.glb assets/models/mita_backup.glb
    fi
else
    echo "Archivo mita_external.glb no encontrado. Usando modelo de respaldo."
fi
