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
        always {
            script {
                if (currentBuild.currentResult == 'FAILURE') {
                    emailext subject: '$DEFAULT_SUBJECT',
                        body: "<b>FAILURE</b><br>Project: ${env.JOB_NAME} <br>Build Number: ${env.BUILD_NUMBER} <br> URL de build: ${env.BUILD_URL}",
                        recipientProviders: [
                            [$class: 'CulpritsRecipientProvider'],
                            [$class: 'DevelopersRecipientProvider'],
                            [$class: 'RequesterRecipientProvider']
                        ],
                        replyTo: '$DEFAULT_REPLYTO',
                        to: '$DEFAULT_RECIPIENTS'
                }
            }
            cleanWs(cleanWhenNotBuilt: false,
                    deleteDirs: true,
                    disableDeferredWipeout: true,
                    notFailBuild: true,
                    patterns: [[pattern: '.gitignore', type: 'INCLUDE'],
                               [pattern: '.propsfile', type: 'EXCLUDE']])
        }
    }
}
