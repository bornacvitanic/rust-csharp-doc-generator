# Fearem [System name] System

## Overview
[Short information packed summary of the system. No need to start it off by repeating the system name, just go immediately into describing what it is. No need to mention it is for Unity, that is self explanatory.]

## Key Interfaces
{% for interface in interfaces %}
- **`{{ interface }}`**: [one_sentence_summary].
  {% endfor %}

## Main Classes
{% for class in classes %}
- **`{{ class }}`**: [one_sentence_summary].
  {% endfor %}

## Structs
{% for struct in structs %}
- **`{{ struct }}`**: [one_sentence_summary].
  {% endfor %}

## Enums
{% for enum in enums %}
- **`{{ enum }}`**: [one_sentence_summary].
  {% endfor %}

## Usage

[Short usage intro explaining how this is a simple example on how the system can be used]:

1. **[First step]**: [first step details].
2. **[Second step]**:
   - [second step details].
   - [second step details].
3. **[Third step]**:
   - [third step details].