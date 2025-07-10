#!/usr/bin/env python3
import sys
from PIL import Image


def read_hex_file(path):
    """
    Lê todo o conteúdo do ficheiro, filtra caracteres hex e retorna lista de bytes.
    """
    data = []
    with open(path, "r") as f:
        for line in f:
            line = line.strip()
            # Processa pares de dígitos hex
            for i in range(0, len(line), 2):
                pair = line[i : i + 2]
                if len(pair) == 2:
                    try:
                        data.append(int(pair, 16))
                    except ValueError:
                        # ignora pares inválidos
                        pass
    return data


def expand_4bit_to_8bit(data4):
    """
    Cada byte contém dois pixels de 4 bits.
    Retorna lista de bytes de 8 bits (um por pixel).
    """
    pixels = []
    for byte in data4:
        hi = (byte >> 4) & 0x0F
        lo = byte & 0x0F
        pixels.append((hi << 4) | hi)
        pixels.append((lo << 4) | lo)
    return pixels


def hex_file_to_image(hex_path, output_basename):
    # 1. Lê e converte hex do ficheiro
    data4 = read_hex_file(hex_path)
    expected = 3200
    if len(data4) < expected:
        raise RuntimeError(f"Lidos {len(data4)} pares hex, esperado {expected}")
    # 2. Expande para lista de 140×140 bytes
    pixels = expand_4bit_to_8bit(data4[:expected])
    # 3. Cria imagem grayscale L
    img = Image.new("L", (80, 80))
    img.putdata(pixels)
    # 4. Salva PNG e BMP
    img.save(f"{output_basename}.png")
    img.save(f"{output_basename}.bmp")
    print(f"Imagens geradas: {output_basename}.png e {output_basename}.bmp")


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Uso: ./hex2img_file.py <ficheiro_hex.txt> <output_basename>")
        sys.exit(1)
    hex_file = sys.argv[1]
    out_base = sys.argv[2]
    hex_file_to_image(hex_file, out_base)
