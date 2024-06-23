import logging

from flask import Flask, jsonify, request

app = Flask(__name__)


@app.route('/heartbeat', methods=['POST'])
def heartbeat():
    app.logger.info(f"Heartbeat request body: {request.json}")
    # TODO: in step 2, change this response yaml
    return jsonify({'status': 'success'})


if __name__ == '__main__':
    app.logger.setLevel(logging.INFO)
    app.run(host="0.0.0.0", debug=True)
