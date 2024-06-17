pipeline {
    agent {
        docker{
            image 'ghcr.io/cargo-lambda/cargo-lambda'
        }
    }
    parameters {
        choice(name: 'LAMBDA', choices:["lambda_create_user"], description: 'Pick a lambda')
    }

    stages {
        stage("Build") {
            steps {
                echo "Build lambda: ${params.LAMBDA}"
                sh "cargo lambda build --release -p '${params.LAMBDA}' --arm"
            }
        }
    }
}