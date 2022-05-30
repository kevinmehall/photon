# Photon

Photon is an ultra-lightweight tool for searching and visualizing logs at the small to medium scale. 

Give Photon some TOML configuration files telling it where to find and how to parse your logs, and it will serve a web interface for analyzing them. You can also query a Photon server via a JSON API, or use Photon's query engine as a command line utility or Rust library without running the server.

Other log analysis systems like Elastic and Loki need to ingest and store your logs; Photon just searches them from files on disk or (soon) from an S3 compatible object store. Photon aims to give you the power of Kibana, without the need to run a heavy stateful service or send your data to a hosted service.

Photon is distributed under the terms of the Mozilla Public License v2, and is written in Rust (query engine and server) and TypeScript + Preact (frontend UI).

## Concepts

Photon searches logs from one or more *datasets* that are collections of log *records*. Records consist of *fields* with *values*. Each dataset is configured by a TOML file in Photon's configuration directory.

The dataset configuration file specify a *source* that provides raw records such as lines from a set of files, and a series of *parsers* specifying how to augment these records by parsing and extracting structured data into additional fields. Queries specify filters on fields to extract and return a subset of the available records.

Unlike JSON, fields form a flat namespace, but we can transform nested JSON structures into fields by expanding the field names with dots. 

