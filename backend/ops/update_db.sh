#!/bin/sh

CURR_DIR=`dirname "$0"`

SKIP_DOCKER=true ${CURR_DIR}/init_db.sh
