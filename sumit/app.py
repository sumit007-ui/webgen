from flask import Flask, render_template, request, jsonify
import os

app = Flask(__name__)

@app.route('/')
def index():
    return render_template('index.html')

@app.route('/about')
def about():
    return render_template('about.html')

@app.route('/projects')
def projects():
    projects_data = [
        {
            'title': 'Project 1',
            'description': 'A cool project built with Python',
            'tech': ['Python', 'Flask', 'JavaScript'],
            'github': 'https://github.com/yourusername/project1'
        },
        {
            'title': 'Project 2',
            'description': 'An awesome web application',
            'tech': ['React', 'Node.js', 'MongoDB'],
            'github': 'https://github.com/yourusername/project2'
        },
    ]
    return render_template('projects.html', projects=projects_data)

@app.route('/contact', methods=['GET', 'POST'])
def contact():
    if request.method == 'POST':
        data = request.form
        # Handle form submission here
        return jsonify({'status': 'success', 'message': 'Message received!'})
    return render_template('contact.html')

if __name__ == '__main__':
    app.run(debug=True, host='0.0.0.0', port=5000)
