This directory contains CSV files representing collections of ODP repositories.  It also contains a Python
script that generates a Leptos component that will render the repositories and their linked relationships
as specified in the CSV file.

The CSV format is as follows, presented as columns starting from the left:

1. Repository name (text string)
2. Repository URL (text string)
3. Child repository list (array of zero-based index numbers), each index is the position of the child repository in the CSV file) 
4. Repo classification (text string), repositories are grouped in columns based on classification
5. Classification order (number), represents the column number where the classification is located

To generate the Leptos component from the CSV file:

`python generate_web_graph.py <CSV filename> <component.rs>`

Note that the Python script also prints condensed repository information and associated links to the console.  This text can be copy-pasted directly into the existing Leptos component file as `nodes_data` and `links_data` (ex: boot_firmware.rs, embedded_controller.rs, unified_ec_services.rs).

2025-07-16
