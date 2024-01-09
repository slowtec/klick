pipeline {
    agent any
    environment {
        PATH="/run/current-system/sw/bin"
    }
    stages {
        stage('Build') {
            steps {
                echo 'Building..'
                sh 'nix-shell --command "just build"'
                archiveArtifacts artifacts: 'frontend/dist/*', fingerprint: true 
            }
        }
        stage('Test') {
            steps {
                echo 'Testing..'
                sh 'nix-shell --command "just test"'
            }
        }
        stage('Check fmt') {
            steps {
                echo 'Checking fmt..'
                  sh 'nix-shell --command "cargo fmt --check"'
                  sh 'nix-shell --command "cd frontend; cargo fmt --check"'
            }
        }
    }
}
