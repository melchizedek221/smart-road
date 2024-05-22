pub struct StatisticsView {
    pub statistics: Statistics,
}

impl StatisticsView {
    pub fn new(statistics: Statistics) -> StatisticsView {
        StatisticsView { statistics }
    }

    pub fn render(&self) {
        // Render des statistiques
    }
}