from flask import Flask, request, make_response
import pickle
import base64

app = Flask(__name__)

@app.route('/')
def home():
    cookie = request.cookies.get('user_session')
    if not cookie:
        user_data = {'name': 'misafir', 'role': 'user'}
        encoded = base64.b64encode(pickle.dumps(user_data)).decode()
        resp = make_response("<h1>Hoşgeldiniz!</h1><p>Session atandı. Sayfayı yenileyin.</p>")
        resp.set_cookie('user_session', encoded)
        return resp

    try:
        data = pickle.loads(base64.b64decode(cookie))
        return f"<h1>Merhaba {data.get('name', 'Bilinmeyen')}, sistemdeki rolün: {data.get('role', 'user')}</h1>"
    except Exception as e:
        return f"<h1>Hata: {e}</h1>"

if __name__ == '__main__':
    app.run(host='127.0.0.1', port=5000)
