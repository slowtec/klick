pipeline {
    agent any
    environment {
        PATH="/run/current-system/sw/bin"
    }
    stages {
        stage('Build') {
            steps {
                echo 'Building..'
                sh 'ls'
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
    post {
         changed {
            script {
                if (currentBuild.currentResult == 'FAILURE') { 
                    emailext subject: '$DEFAULT_SUBJECT',
                        body: '$DEFAULT_CONTENT',
                        recipientProviders: [
                            [$class: 'CulpritsRecipientProvider'],
                            [$class: 'DevelopersRecipientProvider'],
                            [$class: 'RequesterRecipientProvider'] 
                        ], 
                        replyTo: '$DEFAULT_REPLYTO',
                        to: '$DEFAULT_RECIPIENTS'
                }
            }
        }
        always {
            cleanWs(cleanWhenNotBuilt: false,
                    deleteDirs: true,
                    disableDeferredWipeout: true,
                    notFailBuild: true,
                    patterns: [[pattern: '.gitignore', type: 'INCLUDE'],
                               [pattern: '.propsfile', type: 'EXCLUDE']])
        }
    }
}
