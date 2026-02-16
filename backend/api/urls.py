from django.urls import path
from . import views

urlpatterns = [
    path('heartbeat/', views.heartbeat, name='heartbeat'),
    path('health/', views.health_check, name='health_check'),
    path('dashboard/', views.dashboard_stats, name='dashboard_stats'),
    path('auth/login/', views.login, name='login'),
    path('auth/logout/', views.logout, name='logout'),
    path('auth/me/', views.me, name='me'),
]