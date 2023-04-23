workspace {

    model {
        u = person "User"

        s = softwareSystem "Simulation and Visualization" "Written in Python and Rust"{
            scripts = container "Initialization Scripts" "Written in Python and Bash"  {
                generate = component "Generate Seed Data"
                run_simulation = component "Run Simulation"
            }

            data = container "Data" "Directory to save the input and output files" {
                tags "Data"
            }

            src = container "Source Implementation" "Written in Rust"{
                simulation = group "Simulation" {
                    find_collision_between_balls = component "Find Collision Between Balls" {
                        tags "Algorithm"
                    }
                    find_collision_against_wall = component "Find Collision Against Wall"
                    find_earliest_collision = component "Find Earliest Collision"
                    apply_collision = component "Apply Collision"
                    run_loop = component "Run"
                }
                visualization = component "Visualization"
            }
        }

        reporting = softwareSystem "Reporting" "Written in Jupyter Notebooks with Seaborn" "Reporting"

        u -> generate "Runs"
        u -> run_simulation "Runs"
        u -> reporting "Reads the results of the report"
        visualization -> u "Displays the visualization of the simulation"

        reporting -> data "Reads output file"

        scripts -> data "Writes input file with seed data" 

        visualization -> data "Reads input file"

        src -> data "Writes output file"

        run_simulation -> src "Runs simulation"

    }

    views {

        systemContext s {
            include *
            autolayout lr
        }

        container s {
            include *
            autolayout lr
        }

        component scripts {
            include *
            autolayout lr
        }

        component src {
            include *
            autolayout lr
        }

        styles {
            element "Data" {
                background #A1A7AD
                color #ffffff
                shape folder
            }

            element "Reporting" {
                shape WebBrowser
            }

            element "Group" {
                color #ff0000
            }
        
        }

        theme default
    }



}